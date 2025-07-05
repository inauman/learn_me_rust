fn main() {
    vec_intro_fn();
}

fn vec_intro_fn() {
    let _v: Vec<i32> = Vec::new();
    let mut v = vec![0, 1, 2, 3, 4];

    v.push(5);

    println!("v: {:?}", v);

    //third element
    let third = &v[2];
    println!("third: {}", third);

    let third_o: Option<&i32> = v.get(20);
    match third_o {
        Some(third) => println!("third_o: {}", third),
        None => println!("No third_o element"),
    }

    v.push(6);

    let first = &v[0];
    println!("first: {}", first);

    println!("v: {:?}", v);
    println!("first: {}", first);

    for i in &mut v {

        *i += 50;
        println!("i: {}", i);
    }

    #[derive(Debug)]
    enum SpreadSheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let row = vec![
        SpreadSheetCell::Int(3),
        SpreadSheetCell::Float(10.12),
        SpreadSheetCell::Text(String::from("blue")),
    ];

    //println!("row: {:?}", row);

    for cell in &row {
        match cell {
            SpreadSheetCell::Int(value) => println!("{}", value),
            SpreadSheetCell::Float(value) => println!("{}", value),
            SpreadSheetCell::Text(value) => println!("{}", value),
        }
    }
}
