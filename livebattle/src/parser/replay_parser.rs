use wows_replays::{ReplayFile, parse_scripts};

pub fn parse_replay<P: wows_replays::analyzer::AnalyzerBuilder>(
    replay: &std::path::PathBuf,
    processor: P,
) -> Result<(), wows_replays::ErrorKind> {
    let replay_file = ReplayFile::from_file(replay)?;

    //let mut file = std::fs::File::create("foo.bin").unwrap();
    //file.write_all(&replay_file.packet_data).unwrap();

    let datafiles = wows_replays::version::Datafiles::new(
        std::path::PathBuf::from("versions"),
        wows_replays::version::Version::from_client_exe(&replay_file.meta.clientVersionFromExe),
    )?;
    let specs = parse_scripts(&datafiles)?;

    let version_parts: Vec<_> = replay_file.meta.clientVersionFromExe.split(",").collect();
    assert!(version_parts.len() == 4);

    let processor = processor.build(&replay_file.meta);

    // Parse packets
    let mut p = wows_replays::packet2::Parser::new(&specs);
    let mut analyzer_set = wows_replays::analyzer::AnalyzerAdapter::new(vec![processor]);
    match p.parse_packets::<wows_replays::analyzer::AnalyzerAdapter>(
        &replay_file.packet_data,
        &mut analyzer_set,
    ) {
        Ok(()) => {
            analyzer_set.finish();
            Ok(())
        }
        Err(e) => Err(e),
    }
}
