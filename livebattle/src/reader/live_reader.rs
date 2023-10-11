use std::{fs::{read_to_string, File}, io::Read, thread::sleep, time::Duration};
use wows_replays::{parse_scripts, ErrorKind, ReplayMeta};

pub fn parse_live_replay<P: wows_replays::analyzer::AnalyzerBuilder>(
    replay_folder: &std::path::PathBuf,
    processor: P,
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

    let datafiles = wows_replays::version::Datafiles::new(
        std::path::PathBuf::from("versions"),
        wows_replays::version::Version::from_client_exe(&meta.clientVersionFromExe),
    )?;
    let specs = parse_scripts(&datafiles)?;

    let version_parts: Vec<_> = meta.clientVersionFromExe.split(",").collect();
    assert!(version_parts.len() == 4);

    // Setup parser and processor
    let processor = processor.build(&meta);
    let mut p = wows_replays::packet2::Parser::new(&specs);
    let mut analyzer_set = wows_replays::analyzer::AnalyzerAdapter::new(vec![processor]);

    // Keep reading the replay file and sending packets to the analyzer
    let mut temp_replay = File::open(temp_replay).map_err(|_| ErrorKind::TempFilesNotFound)?;
    let mut buffer = [0; 256];
    loop {
        let bytes_read = temp_replay
            .read(&mut buffer)
            .map_err(|_| ErrorKind::TempFilesNotFound)?;
        if bytes_read == 0 {
            continue;
        }

        match p.parse_packets::<wows_replays::analyzer::AnalyzerAdapter>(&buffer[..bytes_read], &mut analyzer_set) {
            Ok(()) => {}
            Err(e) => return Err(e),
        }

        // short delay
        sleep(Duration::from_millis(200));
    }
}
