#[derive(Debug)]

struct Food{
    calories: f32,
    class : String,
    season: String,
}

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}
// Structure data always store on the heap and the data will be move when passed to the function
// Whatever the data is in the structure it will be stored on the heap 
// self, impl, print
impl Food{
    fn impl_print(&self){
        println!("{:?}",self.calories);
    }
} // impl method for struct type
fn main() {
    let mut almond = Food{
        calories : 580.5,
        class : "Dry".to_string(),
        season : "Winter".to_string(),
    };
    println!("{:?}",almond);
    print(&mut almond); // simple function call
    almond.impl_print(); // method function call
    let rect1 = Rectangle { width: 30, height: 50 };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}
fn print(almond:&mut Food){
    println!("{:?}",almond);
}