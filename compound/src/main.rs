fn main() {
    let months = ["January","Fabruary","March","April","May","June","July","August","September","October","November","December"];

    let first = months[0];
    let second = months[1];

    println!("first month is {},second month is {}",first,second);

    let x = plus_five(6);

    let y = {
        let x = 1;
        x+3
    };
    println!("x is {} and y is {}",x,y);
}

fn plus_five(x: i32)->i32{
    x+5
}