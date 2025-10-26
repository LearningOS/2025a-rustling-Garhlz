// move_semantics5.rs
//
// Make me compile only by reordering the lines in `main()`, but without adding,
// changing or removing any of them.
//
// Execute `rustlings hint move_semantics5` or use the `hint` watch subcommand
// for a hint.

fn main() {
    let mut x = 100;
    let y = &mut x;
    // 需要用解引用符号
    *y += 100;
    let z = &mut x;
    *z += 1000;
    // *y += 10;
    // 只能同时存在一个可变引用
    // let k = &x;
    // println!("{}", k);
    // 或者同时存在多个不可变引用

    assert_eq!(x, 1200);
}
