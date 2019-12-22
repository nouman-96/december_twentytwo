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
struct Team{
    name : String,
    score: u32,
}
impl Team{
    fn high(&self,other: &Team) -> u32{
        if self.score > other.score{
            self.score
        }
        else 
        {
            other.score
        }
    }
}
impl Rectangle { //  method is implemented on the data type defined by using self 
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

// difference betwen method and function
/*
method is implemented on the structure define on the function
no change in calling while using method  */

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
    let team1 = Team{name :"Srilanka".to_string(), score : 560};
    let team2 = Team{name : "Pakistan".to_string(), score : 480};
    let high_score:u32 = team1.high(&team2);

    println!("In main : {}",high_score);

}
fn print(almond:&mut Food){
    println!("{:?}",almond);
}