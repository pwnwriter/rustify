pub fn variables() {
    /* Variables can be defined in a single line using the `let` keyword followed by the
    name of the variable and the value it's being assigned to, separated by an equal sign `(=)`. */

    let x = 4;

    let name = "Pwnwriter";

    println!("{x}");

    println!("{}", name);
}

pub fn muts() {
    /*the mut keyword before a variable declaration makes the variable mutable,
    meaning its value can be changed later in the program.  */

    // let a = 5; Doesn't let us define a second variable until and unless it's mutable

    let mut a = 5;
    println!("The value of a is {a}");

    a = 4;
    println!("The value of a is {a}");
}

pub fn constants() {
    /*constants can be defined using the const keyword followed by the name of the constant and its value,
     * separated by an equal sign (=). Constants must be annotated with a type,
     * and the value must be a constant expression: */

    const NAME: &str = "Pwnwriter";
    println!("My name is {NAME}");
}

pub fn shadowing() {
    /*shadowing refers to the practice of declaring a new variable with the same name as a previous variable,
     * which effectively "shadows" the previous variable and makes it inaccessible for the rest of the code block.*/

    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("{x}");

    let l = {
        let m = 3;
        let o = m + 4;
        o + 6
    };

    println!("The final value of l is {}", l);

    //let space = " ";
    //println!("{}", space);
    //println!("{}", space.len());
}

pub fn data_types() {
    /*Generally, data type means specifying the type of the variable and give it an initial value in a single statement.
     * The format for this is: let <variable name>: <data type> = <initial value>; */

    let an_unsigned_integer: u32 = 42;
    let a_float: f64 = 3.14;
    let a_boolean: bool = true;
    let a_character: char = 'z';
    let a_string: &str = "Hello, Pwn!";

    println!("Unsigned Integer: {}", an_unsigned_integer);
    println!("Float: {}", a_float);
    println!("Boolean: {}", a_boolean);
    println!("Character: {}", a_character);
    println!("String: {}", a_string);
}

pub fn tuples() {
    /*Tuples in Rust are collections of values of different data types, grouped together into a single compound value.*/

    let a_tuple: (i32, f64, bool, char, &str) = (42, 3.14, true, 'z', "Hello, Rust!");

    println!("Tuple: {:?}", a_tuple);

    println!("The first element is {}", a_tuple.0);
}

pub fn arrays() {
    /* Arrays in Rust are fixed-size collections of values of the same data type. 
     * They are defined using square brackets and comma-separated values.*/
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    for i in 0..12 {
        println!("{}", months[i]);
    }
}
