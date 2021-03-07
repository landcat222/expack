use std::path::PathBuf;
use std::env;

pub(crate) struct EnvConf {
    expath: PathBuf,
    bin: PathBuf,
    pack: PathBuf,
    base: PathBuf,
    files: PathBuf,
    src: PathBuf,
}

pub(crate) fn init(file: String) -> EnvConf {
    EnvConf {
        expath: get_expath(),
        bin: {
            let mut bin = get_expath();
            bin.push("bin");
            bin
        },
        pack: {
            let mut pack = get_expath();
            pack.push("pack");
            pack
        },
        base: {
            let mut base = get_expath();
            base.push("pack");
            base.push(&file);
            base
        },
        files: {
            let mut files = get_expath();
            files.push("pack");
            files.push(&file);
            files.push("files");
            files
        },
        src: {
            let mut src = get_expath();
            src.push("pack");
            src.push(&file);
            src.push("pack");
            src
        },
    }
}

fn get_expath() -> PathBuf {
    let expath = match env::var("EXPATH") {
        Ok(path) => PathBuf::from(path),
        Err(_) => {
            let mut home = dirs::home_dir().expect("Home directory not found");
            home.push(".expack");
            home
        },
    };
    expath
}
