use symlink::symlink_file;

fn main() {
    println!("Hello, world!");
    // ? Source File src/main.rs and Link File /home/remon/Documents/main2.rs
    symlink_file("src/main.rs", "/home/remon/Documents/main2.rs").unwrap();
}
