use std::fs::Metadata;
use std::os::unix::fs::PermissionsExt;
use crate::util::Group::{Owner, _Group, Other};

enum Group {
    Owner,
    _Group,
    Other
}

enum Perms{
    R,
    W,
    X,
    RW,
    RX,
    WX,
    RWX,
    NULL
}

impl ToString for Perms {

    fn to_string(&self) -> String {
        match self {
            Perms::R => {String::from("r--")}
            Perms::W => {String::from("-w-")}
            Perms::X => {String::from("--x")}
            Perms::RW => {String::from("rw-")}
            Perms::RX => {String::from("r-x")}
            Perms::WX => {String::from("-wx")}
            Perms::RWX => {String::from("rwx")}
            Perms::NULL => {String::from("---")}
        }
    }
}

fn group_vals(group: Group) -> (u32,u32){
    match group {
        Group::Owner => {(6,0o700)}
        Group::_Group => {(3,0o70)}
        Group::Other => {(0,0o7)}
    }
}

pub fn get_perms(perms: u32) -> String{
    let mut string = String::from("");
    let groups = [Owner,_Group, Other];
    for group in groups{
        let perms = perms & 0o777;
        let (shift, mask) = group_vals(group);
        let perms = perms & mask;
        let perms = perms >> shift;
        let perm_string = get_perm_string(perms);
        string += &*perm_string;
    }

    return string;
}

pub fn get_perm_string(perm: u32) -> String{
    let group:Perms = match perm {
        7 => {Perms::RWX}
        6 => {Perms::RW}
        5 => {Perms::RX}
        4 => {Perms::R}
        3 => {Perms::WX}
        2 => {Perms::W}
        1 => {Perms::X}
        _ => {Perms::NULL}
    };

    group.to_string()
}
pub fn is_exe(meta: Metadata)-> bool {
    let mode = meta.permissions().mode();
    let is_file = meta.is_file();

    is_file && mode & 0o111 !=0
}
