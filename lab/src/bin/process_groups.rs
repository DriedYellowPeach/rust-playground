use nix::unistd;

fn main() {
    let pid = unistd::tcgetpgrp(0).unwrap();
    println!("foreground process group is: {pid}");

    let my_grp = unistd::getpgrp();
    println!("my process group is: {my_grp}");
}
