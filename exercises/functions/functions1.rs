// functions1.rs
//
// Execute `rustlings hint functions1` or use the `hint` watch subcommand for a
// hint.

fn main() {
    call_me(10);
}

fn call_me(num: i32) {
    if num == 0 {
        return;
    }
    println!("num is {}", num);
    call_me(num - 1);
}
