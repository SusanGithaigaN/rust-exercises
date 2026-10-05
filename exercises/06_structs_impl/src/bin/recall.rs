// Read ../../README.md before starting.

#[derive(Debug)]
struct Rectangle {
    width: f64,
    height: f64,
}

impl Rectangle {
    fn new(width: f64, height: f64) -> Rectangle {
        todo!()
    }

    fn area(&self) -> f64 {
        todo!()
    }

    fn perimeter(&self) -> f64 {
        todo!()
    }

    fn is_square(&self) -> bool {
        todo!()
    }

    fn scale(&self, factor: f64) -> Rectangle {
        todo!()
    }
}

fn main() {
    let r = Rectangle::new(4.0, 6.0);
    println!("{:?}", r);
    println!("{}", r.area());
    println!("{}", r.perimeter());
    println!("{}", r.is_square());
    println!("{:?}", r.scale(2.0));

    let sq = Rectangle::new(5.0, 5.0);
    println!("{}", sq.is_square());
}

// Supplied checks; leave these unchanged.
#[cfg(test)]
include!(concat!(env!("CARGO_MANIFEST_DIR"), "/checks/recall.rs"));
