#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

#[derive(Debug)]
struct Rectangle {
        width: u32,
        height: u32
    }


//CREATING ASSOCIATED FUNCTIONS AND METHODS (which are assoc functions that take the self param) ON A STRUCT.

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size
        }
    }
}

fn main() {
    println!("Hello, world!");
    let user = return_struct(String::from("Tamarah"), String::from("paulorubebe@yahoo.com"));
    println!("The struct is {:#?}", user);

    let width = 30;
    let height = 40;
    let rect1 = (30, 40);
    let scale = 2;
    
    let rect = Rectangle {
        width: 40,
        height: 70
    };

    let rect2 = Rectangle {
        width: dbg!(30 * scale),
        height: 40
    };

    let rect3 = Rectangle {
        width: 30,
        height: 60
    };

    let sq = Rectangle::square(40);

   println!("The area of the rectangle is {} square pixels", area(width, height));
   println!("The area of the rectangle is {} square pixels", area1(rect1));
   println!("The area of the rectangle is {} square pixels", area2(&rect2));
   println!("The area of the rectangle is {} square pixels", rect3.area());
   println!("can rect hold rect3? {} ", rect.can_hold(&rect3));
   println!("The area of the square is {} ", sq.area());


   dbg!(&rect2);
    
}


fn return_struct(username: String, email: String) -> User {
     User {
        username,
        email,
        active: true,
        sign_in_count: 1
    }
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

// refractoring using tuples
fn area1(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

//refractoring using structs
fn area2(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
