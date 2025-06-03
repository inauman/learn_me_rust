fn main() {
    let width1 = 30;
    let height = 50;

    let area = area(width1, height);

    println!("The area of the rectangle is {} square pixels.", area);


}


fn area(width: u32, height: u32) -> u32 {
    width * height
}