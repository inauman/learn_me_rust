fn main() {
    
    let mut s = String::from("Hello World!");
    let word = first_word(&s);
    println!("The first word is: {word}");

    s.clear();
    //println!("The first word is: {word}");

}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate(){
        if item == b' ' {
            return i;
        }
    }
    s.len()
}