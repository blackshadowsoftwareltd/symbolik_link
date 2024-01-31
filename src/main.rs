use symlink::symlink_file;

fn main() {
    println!("Symbolic Linking in Rust");
    // ? Source File src/main.rs and Link File /home/remon/Documents/main2.rs
    match symlink_file("Cargo.toml", "/home/remon/Documents/main2.rs") {
        Ok(_) => println!("Symlink created"),
        Err(e) => println!("Error: {}", e),
    }
}
