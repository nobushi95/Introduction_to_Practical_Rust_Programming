fn print(s: &[u8]) {
    println!("{:?}", s);
}

fn print_box(s: Box<[u8]>) {
    println!("{:?}", s);
}

fn main() {
    let byte_array = [b'h', b'e', b'l', b'l', b'o'];
    print(&byte_array);
    print_box(Box::new(byte_array));
}
