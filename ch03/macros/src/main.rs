use std::io::Write;
fn main() {
    let mut w = Vec::new();
    write!(&mut w, "{}", "ABC");
    writeln!(&mut w, " is 123");
    dbg!(w);

    println!("conat: {}", concat!("A", "b2", 3));

    println!("defined in file: {}", file!());
    println!("defined in line: {}", line!());
    println!("is test: {}", cfg!(unix));
    println!("CARGO_HOME: {}", env!("CARGO_HOME"));
}
