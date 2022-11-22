fn main() {
    let dep_version = dep_1::get_version();
    println!("Hello, world! (lib 2.3, base " + dep_version + ")");
}
