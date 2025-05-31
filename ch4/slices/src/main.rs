fn main() {
    
    let s = String::from("Hello World!How are!");
    let word = first_word_index(&s);
    println!("The first word is: {word}");

    //s.clear();
    //println!("The first word is: {word}");

    let word1 = first_word_slice(&s);
    println!("The first word is: {word1}");

    let word2 = second_word_slice(&s);
    println!("The second word is: {word2}");

}

fn first_word_index(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate(){
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

fn first_word_slice(s: &String) -> &str {

    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate(){
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}

fn second_word_slice(s: &String) -> &str {
    let bytes = s.as_bytes();
    let mut first_space = false;
    let mut m = 0;
    let mut n = 0;
    for (i, &item) in bytes.iter().enumerate(){
        if item == b' ' {
            if first_space {
                n = i;  //second space  
                return &s[m+1..n];
            } else {
                first_space = true;
                m = i;
            }
        }
    }
    &s[m+1..]
}