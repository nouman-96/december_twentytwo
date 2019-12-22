#[derive(Debug)]

struct Food{
    calories: f32,
    class : String,
    season: String,
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
}
fn print(almond:&mut Food){
    println!("{:?}",almond);
}