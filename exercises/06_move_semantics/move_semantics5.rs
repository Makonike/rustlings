// move_semantics5.rs
//
// Make me compile only by reordering the lines in `main()`, but without adding,
// changing or removing any of them.
//
// Execute `rustlings hint move_semantics5` or use the `hint` watch subcommand
// for a hint.

#[test]
fn main() {
    let mut x = 100;
    let y = &mut x;
    *y += 100; // 只存在一个可变引用。在 Rust 中，同时拥有多个可变引用指向同一个变量是不允许的，因为这可能导致数据竞争和未定义的行为。
    let z = &mut x;
    *z += 1000;
    assert_eq!(x, 1200);
}
