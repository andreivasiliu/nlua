use hlua::{Lua, function0};

fn rust_helloer() -> i32 {
    println!("Hello!");
    return 3;
}

fn the_thing() -> i32 {
    let mut lua = Lua::new();

    lua.set("x", 2);
    lua.set("hello", function0(rust_helloer));

    let y = lua.execute("return x + hello();").unwrap();

    y
}

fn main() {
    let y = the_thing();

    println!("Result: {}", y);
}

#[cfg(test)]
#[test]
pub fn test_the_thing() {
    assert_eq!(the_thing(), 5);
}
