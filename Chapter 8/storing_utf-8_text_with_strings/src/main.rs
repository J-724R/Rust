fn main() {
    println!("Hello, world!");

    let v: Vec<i32> = Vec::new();

    let v = vec![1, 2, 3];

    // The most common way to create a vector is with the generic type Vec<T>
    // or with the macro vec!

    // Updating a Vector
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    // reading elements of vectors

    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    let v = vec![1, 2, 3, 4, 5];

    // let does_not_exist = &v[100];
    // let does_not_exist = v.get(100);

    // iterating over the values in a vector
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }

    // Using enums to store multiple types
    #[derive(Debug)]

    enum SpreadSheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    println!("\nSpread Sheet Cell\n");
    let row = vec![
        SpreadSheetCell::Int(3),
        SpreadSheetCell::Text(String::from("blue")),
        SpreadSheetCell::Float(10.12),
        SpreadSheetCell::Text(String::from("Ryuk")),
    ];

    for i in &row {
        println!("{:#?}", &i);
    }

    //Dropping a Vector Drops its elements
    {
        let v = vec![1, 2, 3, 4];

        // do stuff with v
    } // <- v goes out of scope and is freed here
}
