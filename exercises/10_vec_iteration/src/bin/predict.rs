// Read ../../README.md before starting.

fn main() {
    let v = vec![1, 2, 3, 4, 5];

    let doubled: Vec<i32> = v.iter().map(|x| x * 2).collect();
    println!("{:?}", doubled);

    let evens: Vec<&i32> = v.iter().filter(|x| *x % 2 == 0).collect();
    println!("{:?}", evens);

    let sum: i32 = v.iter().sum();
    println!("sum: {}", sum);

    println!("v still here: {:?}", v);

    let v2 = vec![10, 20, 30];
    let doubled2: Vec<i32> = v2.into_iter().map(|x| x * 2).collect();
    println!("{:?}", doubled2);
    // println!("{:?}", v2);

    let result: Vec<i32> = vec![1, 2, 3, 4, 5, 6]
        .iter()
        .filter(|&&x| x % 2 == 0)
        .map(|&x| x * x)
        .collect();
    println!("{:?}", result);
}
