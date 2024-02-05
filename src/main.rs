// An example of imlementing methods for structs

// Our struct for implementing (the [derive(Debug)] attribute is for text output)
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// An example of implementing a method on a struct
impl Rectangle {
    // Methods can take ownership of self, borrow self immutably, as we’ve done here, or borrow self mutably, 
    // just as they can any other parameter.
    fn area(&self) -> u32 { // methods must reference &self first
        self.width * self.height
    }

    // Other methods go here:

    // A method that returns "true" if the rectangle has a non-zero width
    fn width(&self) -> bool { // methods can have the same name as properties of the struct, usually as getters
        self.width > 0
    }

    // A method that checks whether another rectangle can fit in this one
    fn can_fit(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }

    // This is an "associated function":
    // Note that it doesn't reference "self" in its parameters
    // In this case, it acts as a constructor for a certain type of Rectangle
    fn square(length: u32) -> Self { 
        Self {
            width: length,
            height: length,
        }
    }

    // This acts as a constructor for any rectangle. "new" is not a special word in Rust
    fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
        }
    }
}

// Multiple "impl" blocks can be associated with any struct. This is useful with generics, which we'll get to in Ch 10
impl Rectangle {
    fn _example(&self) {}
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area() // An example of calling a method on a struct
    );

    // The second method 
    if rect1.width() {
        println!("rect1 has a non-zero width of {}", rect1.width);
    }

    let rect2 = Rectangle {
        width: 10,
        height: 40
    };

    // Making rectangles with our associated functions:
    let rect3 = Rectangle::new(60, 45);
    let _square = Rectangle::square(30);

    println!("Can rect1 hold rect2? {}", rect1.can_fit(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_fit(&rect3));

}
