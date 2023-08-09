use std::fs;
use std::fs::{Metadata};
use std::os::unix::prelude::{PermissionsExt};

enum Group {
    Owner,
    Group,
    Other
}

fn group_vals(group: Group) -> (u32,u32){
    match group {
        Group::Owner => {(6,0o700)}
        Group::Group => {(3,0o70)}
        Group::Other => {(0,0o7)}
    }
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

fn get_perms(perms: u32,group: Group) -> u32{
    let perms = perms & 0o777;
    let (shift, mask) = group_vals(group);
    let perms = perms & mask;
    let perms = perms >> shift;
    return perms
}

fn get_perm_string(perm: u32) -> String{
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

fn is_exe(meta: Metadata)-> bool {
    let mode = meta.permissions().mode();
    let is_file = meta.is_file();

    is_file && mode & 0o111 !=0
}

fn main() {
    let dir = fs::read_dir(".").unwrap();
    for entry in dir {
        let entry = entry.unwrap();
        let meta = entry.metadata().unwrap();
        let permissions = meta.permissions().mode();


        let owner_perms = get_perm_string(get_perms(permissions, Group::Owner));
        let group_perms = get_perm_string(get_perms(permissions, Group::Group));
        let other_perms = get_perm_string(get_perms(permissions, Group::Other));
        let all_perms = owner_perms + &*group_perms + &*other_perms;
        println!("{} {} {}",entry.path().display(), all_perms, is_exe(meta));
    /*
        100000111101101

        XXXXXX111101101
     */

    }
}
