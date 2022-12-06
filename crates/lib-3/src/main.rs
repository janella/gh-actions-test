fn main() {
    let dep_version = dep_1::get_version();
    println!("Hello, world! (lib 3v0, base {})", dep_version);
}
