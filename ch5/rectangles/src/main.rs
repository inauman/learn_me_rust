// define rectangle struct
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let width1 = 30;
    let height = 50;

    let area = area(width1, height);

    println!("The area of the rectangle is {} square pixels.", area);

    let rect1 = (width1, height);
    println!("Rectangle tuple: {:#?}", rect1);

    let area_tuple_a = area_tuple(rect1);
    println!("Tuple:The area of the rectangle is {} square pixels.", area_tuple_a);
  
    let rect2 = Rectangle {
        width: 30,
        height: 50,
    };

   

    let area_struct_a = area_struct(&rect2);
    println!("Struct: The area of the rectangle is {} square pixels.", area_struct_a);

    println!("Rectangle struct: {rect2:#?}");
   println!("\n--------------------------------\n");
    // using debug macro

    //let scale = 2;
    let rect31 = Rectangle {
        width: dbg!(30 * 2),
        height: 50,
    };
    dbg!(&rect31);
   
    let area31 = area_struct(&rect31);
    println!("Area of rect3: {}", area31);
    
}


fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area_struct(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}