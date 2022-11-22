fn main() {
    let dep_version = dep_1::get_version();
    println!("Hello, world! (lib 1.5, base {})", dep_version);
}
