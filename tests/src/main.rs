fn main() {
    let v = vec!["1", "2", "3"];
    println!("{}", v[1]);
    for i in &v {
        // Either clone or use reference operator
        println!("{}", i);
    }
    println!("{}", v[1]);
    test();
}

/*
The code attempts to print the second element of a
vector v after iterating over its elements in a loop. However,
because the loop takes ownership of the vector and leaves it empty
afterwards, the attempt to print the second element results in a runtime error.
To fix this, you can clone the vector before iterating over it or use a reference to the vector in the loop.
*/

fn test() {
    let a = 1;
    println!("{:p}", &a)
}
