// Dangling references
fn main() {
    let ref_value = no_dangle();
    println!("{}", ref_value);
}

fn no_dangle() -> String {
    let s = String::from("hello");
    s
} 

