/*a function in Rust is a reusable block of code that takes in certain inputs, performs operations on them,
 * and then returns an output. Rust supports writing functions in a concise way, where the entire function definition
 * can be written in a single line. This is done by providing the inputs, operations, and the output type,
 * all in a compact syntax.
*/
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn functions() {
    println!("The result is: {}", add(4, 5));
}

// -------------- Comments ------------//

pub fn comments() {
    /*multiline
     * comments */
    // single line comments
    println!("Hello from comments");
}

// --------------- Expressions ----------//

/*an expression is a piece of code that returns a value. An expression can be as simple as a single value (e.g. 5),
or as complex as a combination of operations and function calls (e.g. a + b * c). Expressions are a fundamental
building block of the language and are used to write complex logic, perform arithmetic operations, and determine
the flow of control in a program. In Rust, almost everything is an expression, including if-else statements,
match expressions, and even blocks of code.*/

pub fn expressions() {
    let x = 100;
    if x > 5 {
        println!("It's true than x is greater than 5");
    } else {
        println!("It's false");
    }
}

pub fn loops() {
    //loop {
    //    println!("Hello it's loop");
    //}

    //for _i in 1..3 {
    //  println!("Hello it's loop");
    //}
    // let i = 1;

    // while  i < 1 {
    //     println!("Hello it's loop");
    // }

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("The value is: {}", a[index]);

        index += 1;
    }
}

