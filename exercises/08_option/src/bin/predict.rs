// Read ../../README.md before starting.

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

    let result = Some(10).map(|n| n + 5).map(|n| n * 2);
    println!("{:?}", result);
}
