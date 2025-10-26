// primitive_types5.rs
//
// Destructure the `cat` tuple so that the println will work.
//
// Execute `rustlings hint primitive_types5` or use the `hint` watch subcommand
// for a hint.

fn main() {
    let neko = ("hkwxsl", "xiaoyuan");
    let (nickname, realname) = neko;
    println!("{} is also known as {}", nickname, realname);
}
