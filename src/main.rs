trait T2: v::T1 {
    fn a(self) -> u32;
}

#[derive(Clone)]
struct S;

impl v::T0 for S {}
impl v::T1 for S {}
impl T2 for S {
    fn a(self) -> u32 {
        4
    }
}

fn f(a: Box<dyn T2>) {}

fn main() {
    println!("Hello, world!");
}
