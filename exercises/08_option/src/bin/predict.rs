// TOPIC 08 — Option<T>
// EXERCISE TYPE: Predict
// =============================================
// BEFORE running, write down the output of each println.
// Pay attention to: Some vs None, .map(), .unwrap_or(), if let.
//
// Questions to answer first:
//   1. What does .map() do to a None value?
//   2. What does .unwrap_or(0) do when the Option is None?
//   3. What is the type of `doubled` after .map()?
//
// Run: cargo run --bin predict

fn main() {
    let x: Option<i32> = Some(5);
    let doubled = x.map(|n| n * 2);
    println!("{:?}", doubled);

    let nothing: Option<i32> = None;
    let also_nothing = nothing.map(|n| n * 2);
    println!("{:?}", also_nothing);

    println!("{}", nothing.unwrap_or(0));
    println!("{}", x.unwrap_or(0));

    let name: Option<&str> = Some("alice");
    if let Some(n) = name {
        println!("name is: {}", n);
    }

    let no_name: Option<&str> = None;
    if let Some(n) = no_name {
        println!("name is: {}", n);
    } else {
        println!("no name");
    }

    let v = vec![1, 3, 5, 6, 8];
    let first_even = v.iter().find(|&&x| x % 2 == 0);
    println!("{:?}", first_even);

    // What does this chain produce?
    let result = Some(10)
        .map(|n| n + 5)
        .map(|n| n * 2);
    println!("{:?}", result);
}
