use std::path::PathBuf;
use crate::init;

fn format(src: String) -> String {
    let pieces = src.split("\n");
    let mut result = String::new();
    for i in pieces {
        if i.trim() != "" {
            result = format!("{}\n{}",result,i.trim());
        }
    }
    result
}

fn conv_token(src: String) -> Vec<Vec<String>> {
    let mut token: Vec<Vec<String>> = Vec::new();
    for i in src.split("\n") {
        let mut tmp = Vec::new();
        for j in i.split(" ") {
            tmp.push(j.to_string());
        }
        token.push(tmp);
    }
    token
}

pub(crate) fn get_src(src: init::EnvConf) -> Vec<Vec<String>> {
    conv_token(format(src.get_src()))
}
