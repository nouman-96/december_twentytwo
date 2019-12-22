#[derive(Debug)]

struct Food{
    calories: f32,
    class : String,
    season: String,
}
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
#[derive(Debug)]
struct Team{
    name : String,
    score: u32,
}
#[derive(Debug)]
struct Foody{
    name : String,
    price : u16,
    serving: u8,
}
impl Team{
    fn high(self,other:Team) -> Team{
        if self.score > other.score{
            self
        }
        else 
        {
            other
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
    let h= team1.high(team2);

    println!("{:?}",h);
    let Foody_1 = UDF("Banana".to_string(),50,12);
    println!("{:?}",Foody_1);

}
fn print(almond:&mut Food){
    println!("{:?}",almond);
}
fn UDF (name:String,price:u16,serving:u8) -> Foody{
    Foody{
        name,
        price,
        serving,
    }

}