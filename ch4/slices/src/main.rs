fn main() {
    
    let s = String::from("Hello World!How are!");
    let word = first_word_index(&s[..]);
    println!("The first word is: {word}");

    //s.clear();
    //println!("The first word is: {word}");

    let word1 = first_word_slice(&s[..5]);
    
    println!("The first word is: {word1}");
    

    let word2 = second_word_slice(&s[..5]);
    println!("The second word is: {word2}");
    
    array_slice();
}

fn first_word_index(s: &str) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate(){
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

fn first_word_slice(s: &str) -> &str {

    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate(){
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}

fn second_word_slice(s: &str) -> &str {
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
    &s[n..]
}

fn array_slice() {
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);
    println!("slice: {:?}", slice);
}