trait Foo {
    fn method(&self) -> String;
}

impl Foo for u8 {
    fn method(&self) -> String { format!("u8: {}", *self) }
}

impl Foo for String {
    fn method(&self) -> String { format!("String: {}", *self) }
}

fn do_something<T>(x: T) where T: Foo {
    x.method();
}

fn main() {
    let x = 5u8;
    let y = "Hello".to_string();
    do_something(x);
    do_something(y);
}
