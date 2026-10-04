fn main(){
    greet("xxxx");

    println!("{}", add(10, 20));
}

fn greet(name: &str){
    println!("Hello {}", name)
}

fn add(a: i32, b: i32) -> i32{ //have to define the return type asw
    a + b //any line without a semicolon is an expression - its value automatically becomes the return value
    //any line with a semicolon is a statement
}