use std::path::PathBuf;
use crate::init;

type Src = Vec<Vec<String>>;

fn format(src: String) -> String {
    let pieces = src.split("\n");
    let mut result = String::new();
    for i in pieces {
        if i.trim() != "" {
            result = format!("{}{}\n",result,i.trim());
        }
    }
    result
}

fn conv_token(src: String) -> Src {
    let mut token: Src = Vec::new();
    for i in src.split("\n") {
        let mut tmp = Vec::new();
        for j in i.split(" ") {
            tmp.push(j.to_string());
        }
        token.push(tmp);
    }
    token
}

pub(crate) fn get_src(src: init::EnvConf) -> Src {
    conv_token(format(src.get_src()))
}
