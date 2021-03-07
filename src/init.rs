use std::path::PathBuf;

struct EnvConf {
    expath: PathBuf,
    bin: PathBuf,
    pack: PathBuf,
    base: PathBuf,
    files: PathBuf,
    src: PathBuf,
}

fn init(expath: PathBuf,file: String) -> EnvConf {
    EnvConf {
        expath: PathBuf.from("/home/foo/.expack"),
        bin: PathBuf.from("/home/foo/.expack/bin"),
        pack: PathBuf.from("/home/foo/.expack/pack"),
        base: PathBuf.from("/home/foo/.expack/pack/bar"),
        files: PathBuf.from("/home/foo/.expack/pack/bar/files"),
        src: PathBuf.from("/home/foo/.expack/pack/bar/pack"),
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
