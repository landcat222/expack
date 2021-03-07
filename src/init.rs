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
