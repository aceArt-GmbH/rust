pub fn foo(num: i32) -> i32 {
    let foo: i32::from_be(num);
    //~^ ERROR expected type, found local variable `num`
    //~| ERROR argument types not allowed with return type notation
    //~| ERROR ambiguous associated type
    foo
}

fn main() {
    let _ = foo(42);
}
