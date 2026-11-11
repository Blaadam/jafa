// similar name to the command but helps with building encase scaffolding is updated and not picked up
fn main() {
    println!("cargo:rerun-if-changed=scaffolding");
}
