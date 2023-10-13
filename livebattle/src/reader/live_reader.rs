use log::{debug, info};
use std::{
    fs::{read_to_string, File},
    io::{Read, Seek},
    path::PathBuf,
    sync::mpsc::Sender,
    thread::sleep,
    time::Duration,
};
use wows_replays::{
    analyzer::Analyzer,
    parse_scripts,
    version::{Datafiles, Version},
    ErrorKind, ReplayMeta,
};

use crate::server::websocket_server::{PacketSender, WebsocketServer};

/// Start the live battle server, and keep reading the replay file and sending packets to the analyzer
/// - ip_address: ip address to start the server
/// - replay_folder: path to the replay folder
/// - delay: delay between reading packets
/// - error_delay: delay between reading packets if there was an error
pub fn start_live_battle_server(
    ip_address: &str,
    replay_folder: &str,
    delay: u64,
    error_delay: u64,
) {
    // start the server and pass down the channel
    let full_address = format!("{}:8615", ip_address);
    let server = WebsocketServer::new(full_address);
    let sender = server.start();

    let replay_folder = PathBuf::from(replay_folder);
    debug!("replay_folder: {:?}", replay_folder);
    loop {
        let result = parse_live_replay_loop(&replay_folder, delay, error_delay, &sender);
        match result {
            Ok(_) => debug!("ok"),
            Err(e) => match e {
                ErrorKind::TempFilesNotFound => {
                    info!("waiting for new replay... stop manually with Ctrl + C...");
                    sleep(Duration::from_millis(5000));
                }
                ErrorKind::DatafileNotFound { version, path: _ } => {
                    info!("Your scripts is outdated, current version is {}, download at https://github.com/wowsinfo/data", version.to_path());
                    break;
                }
                _ => {
                    println!("Please log an issue at https://github.com/wowsinfo/WoWs-LiveBattle with the following error: {}", e);
                    break;
                }
            },
        }
    }
}

fn parse_live_replay_loop(
    replay_folder: &PathBuf,
    delay: u64,
    error_delay: u64,
    sender: &Sender<String>,
) -> Result<(), wows_replays::ErrorKind> {
    // find temp.wowsreplay and tempArenaInfo.json
    let temp_replay = replay_folder.join("temp.wowsreplay");
    let meta_file = replay_folder.join("tempArenaInfo.json");

    // if either file doesn't exist, return error
    if !temp_replay.exists() || !meta_file.exists() {
        return Err(ErrorKind::TempFilesNotFound);
    }

    // read meta file and decode it from json
    let meta = read_to_string(meta_file).map_err(|_| ErrorKind::InvalidArenaJson)?;
    let meta =
        serde_json::from_str::<ReplayMeta>(&meta).map_err(|_| ErrorKind::InvalidArenaJson)?;

    let version = Version::from_client_exe(&meta.clientVersionFromExe);
    let datafiles = Datafiles::new(PathBuf::from("versions"), version)?;
    let specs = parse_scripts(&datafiles)?;

    let version_parts: Vec<_> = meta.clientVersionFromExe.split(",").collect();
    if version_parts.len() != 4 {
        return Err(ErrorKind::InvalidArenaJson);
    }

    // Setup parser and processor
    let processor = PacketSender::new(version, sender.clone(), Some("live.json"));
    let processor = Box::from(processor) as Box<dyn Analyzer>;
    let mut p = wows_replays::packet2::Parser::new(&specs);
    let mut analyzer_set = wows_replays::analyzer::AnalyzerAdapter::new(vec![processor]);

    // Keep reading the replay file and sending packets to the analyzer
    let mut temp_replay_file = File::open(temp_replay).map_err(|_| ErrorKind::TempFilesNotFound)?;
    const BUFFER_SIZE: usize = 20000;
    let mut buffer = [0; BUFFER_SIZE];
    let mut offset = 0;
    loop {
        let position = temp_replay_file
            .seek(std::io::SeekFrom::Current(0))
            .unwrap();

        let result = temp_replay_file.read_exact(&mut buffer[offset..BUFFER_SIZE]);
        if result.is_err() {
            // ignore any errors and continue, we only want to send any valid packets is there are any
            info!("error: {:?}", result);
            // read the temp_replay file again and move to the last position
            let temp_replay = replay_folder.join("temp.wowsreplay");
            // get current localition from temp_replay_file
            let mut length = temp_replay_file.metadata().unwrap().len();

            // keep waiting until the file has more data
            loop {
                if length - position > BUFFER_SIZE as u64 {
                    break;
                }

                sleep(Duration::from_millis(error_delay));
                length = temp_replay_file.metadata().unwrap().len();
                if length < position {
                    // the replay file has been reset
                    return Err(ErrorKind::TempFilesNotFound);
                }
                info!("length: {}, {}", length, length - position)
            }

            temp_replay_file = File::open(temp_replay).map_err(|_| ErrorKind::TempFilesNotFound)?;
            temp_replay_file
                .seek(std::io::SeekFrom::Start(position))
                .unwrap();

            sleep(Duration::from_millis(error_delay));
            continue;
        }

        let bytes_read = BUFFER_SIZE - offset;
        if bytes_read == 0 {
            continue;
        }

        offset = p.parse_live_packets::<wows_replays::analyzer::AnalyzerAdapter>(
            &buffer[..],
            &mut analyzer_set,
        );
        debug!("bytes_read: {}, valid_bytes: {}", bytes_read, offset);

        // no valid packets anymore, copy the rest of the buffer to the beginning
        if offset == 0 {
            // this should never happen
            buffer.copy_within(BUFFER_SIZE - bytes_read..BUFFER_SIZE, 0);
            buffer[BUFFER_SIZE - offset..BUFFER_SIZE].fill(0);
            offset = bytes_read;
        } else {
            // shift remaining bytes to the beginning of the buffer
            buffer.copy_within(offset..BUFFER_SIZE, 0);
            // the clear the rest of the buffer
            buffer[BUFFER_SIZE - offset..BUFFER_SIZE].fill(0);
            offset = BUFFER_SIZE - offset;
        }

        // short delay
        sleep(Duration::from_millis(delay));
        debug!("offset: {}", offset)
    }
}
