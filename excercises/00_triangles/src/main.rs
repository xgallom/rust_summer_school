mod triangle;

use crate::triangle::Triangle;

fn main() {
    let triangle = Triangle::read_from_console();

    println!("Triangle sides: {:?}", triangle.sides());

    if !triangle.is_valid() {
        println!("Triangle inequality does not hold");
        return;
    }

    println!("Triangle perimeter: {}", triangle.perimeter());

    if triangle.is_right() {
        println!("Triangle is Right");
    }

    println!("Triangle is {:?}", triangle.triangle_type());
}
