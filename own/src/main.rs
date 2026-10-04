fn main(){
    let s1 = String::from("Hello");

    let s2 = s1;

    println!("{}", s1);
    println!("{}", s2); //s1 is now redundant, the value is completely copied to s2

    //you can clone it instead

    //let s2 = s1.clone();
}