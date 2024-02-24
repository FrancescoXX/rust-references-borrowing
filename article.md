```rust
//ownership and functions
fn main(){
    let i = 5;
    call_integer(i);
    println!("{}", i);

    let s = String::from ("Hello, World!");
    call_string(s);
    println!("{}", s);
}


fn call_integer(i: i32){
    println!("{}", i);
}

fn call_string(s: String){
    println!("{}", s);
}
```

```rust
//ownership and functions
fn main(){
    let s1 = give_ownership();
    let s2 = String::from("hello");  // s2 comes into scope
    let s3 = take_and_give_back(s2); // s2 is moved into take_and_give_back, it returns s3

    println!("s1: {}", s1);
    println!("s2: {}", s2); // error: value used here after move
    println!("s3: {}", s3);
}

fn give_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn take_and_give_back(a_string: String) -> String {
    a_string
}
```

A possible solution is to use a tuple: we return the string and the length together back.

```rust
fn main(){
    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);
    println!("The length of '{}' is {}", s2, len);
}

fn calculate_length(s: String) -> (String, usize){
    let length = s.len();
    (s, length)
}
```

Anyway, this solution is tedious and error-prone. Rust has a feature called references that allows you to refer to some value without taking ownership of it. 

```rust
fn main(){
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}", s1, len);
}

fn calculate_length(s: &String) -> usize{
    let length = s.len();
    length
}
```

```rust
//mutable references
fn main(){
    let mut s = String::from("hello");
    modify(&mut s);
    println!("{}", s);
}

fn modify(s: &mut String){
    s.push_str(" world");
}
```

```rust
fn main() {
    let mut s = String::from("hello");

    let s1 = &s; //immutable borrow
    let s2 = &s; //immutable borrow

    let s3 = &mut s; //mutable borrow

    println!("{}, {}, {}", s1, s2, s3);
    //throws error: cannot borrow `s` as mutable because it is also borrowed as immutable
}

BUT

```rust
fn main() {
    let mut s = String::from("hello");

    let s1 = &s; //immutable borrow
    let s2 = &s; //immutable borrow

    println!("{} and {}", s1, s2);

    let s3 = &mut s; //mutable borrow

    println!("{}", s3);
}
```

This code is valid because the immutable borrows are not used after the mutable borrow. 


Dangling references
```rust
fn main() {
    let s  = dangle();
}

fn dangle() -> &String {
    let s = String::from("hello");
    &s
}
```


To solve this problem, we can return the string directly. 

```rust
fn main() {
    let s = no_dangle();
}

fn no_dangle() -> String {
    let s = String::from("hello");
    s
}
```
