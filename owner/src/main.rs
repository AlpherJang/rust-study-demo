fn main() {
    let mut s=String::from("Hello");
    s.push_str(", World");
    println!("{}",s);
    take_ownership(s);

    let x = 5;
    makes_copy(x);
    println!("x: {}",x);

    let s1 = gives_ownership();
    println!("s1:{}",s1);

    let s2=String::from("test s2");

    let s3=takes_and_giveback(s2);

    println!("s3:{}",s3)

}

fn take_ownership(some_thing: String){
    println!("{}",some_thing)
}

fn makes_copy(some_number: i32){
    println!("{}",some_number);
}

fn gives_ownership()->String{
    let some_string=String::from("test");
    some_string
}

fn takes_and_giveback(some_string: String)->String{
    some_string
}