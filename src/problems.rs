use std::io;
use std::path::PathBuf;
use std::collections::HashSet;
use read_elmi;
use files;
use cli;
use exposed_tests;

#[derive(Debug)]
pub enum Problem {
    MissingElmJson,
    InvalidCwd(io::Error),
    ChDirError(io::Error),
    ReadTestFiles(io::Error),
    NoTestsFound(HashSet<PathBuf>),

    // Reading elm.json
    ReadElmJson(files::ElmJsonError),

    // Running elm make
    SpawnElmMake(io::Error),
    CompilationFailed(io::Error),

    // Running node
    SpawnNodeProcess(io::Error),

    // Problems from other modules
    ReadElmi(read_elmi::Problem),
    Cli(cli::Problem),
    ExposedTest(exposed_tests::Problem),
}
