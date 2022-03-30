use pyla::ingestion;
use ingestion::*;
fn main() {
    println!("Hello, world!");
    let p1 = PylaId::new(1);
    let p2 = PylaId::new(1);
    let p3 = PylaId::new(3); 
    println!("Test Comparison: {}", p1 > p2);
    println!("Test Comparison: {}", p2 == p3);
    println!("Test Comparison: {}", p3 == p3);
}
