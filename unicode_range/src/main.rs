use std::char;

fn main() {
    for n in 0x0000..0x0fff  {
        if let Some(c) = char::from_u32(n) {
            print!("{}", c);
        }
    }
}
