// https://blog.thoughtram.io/lifetimes-in-rust/
// two lifetimes - allows x to be used as its lifetime is different to the unused/dropped y

struct Point<'a, 'b> {
    x: &'a i32,
    y: &'b i32
}

fn main() {
    let x = 3;
    let r;
    //let u;
    {
        let y = 4;
        let point = Point { x: &x, y: &y };
        r = point.x;
        //u = point.y
    }
    println!("{}", r);
    //println!("{}", u);
}