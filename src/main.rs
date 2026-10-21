use std::io;

//defining calculative functions 

fn add(a:i32,b:i32)->i32{
    a+b
}
fn sub(a:i32,b:i32)-> i32{
    a-b
}
fn mul(a:i32,b:i32)-> i32{ 
    a*b
}
fn div(a:i32,b:i32)-> i32{
    a/b
}


fn main(){

    println!("welcome to the rust calculatorrr");
    //starting the calculator loop 
    loop{
    println!("enter the first number");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("hehe");
    let a : i32 = input1.trim().parse().expect("huhu");
    println!("enter the second number");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("hehe");
    let b : i32 = input2.trim().parse().expect("huhu");
    let n = add(a,b);
    let  m = sub(a,b);
    let l = mul(a,b);
    let o = div(a,b);

    println!("1 -> addition \n 2 -> subtraction \n 3 -> multiplication \n 4 -> division");

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("hehe");
    let c:i32 = choice.trim().parse().expect("huhuh");
    let result = match c {
        1 => n,   
        2 => m,
        3 => l,     
        4 => o,
        _ => {println!("hehe");
            return;}
    };
    println!("the answer is {result}");
    println!("enter q to quit or enter c to continue");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("hehe");

    if input.trim() == "q" {
        println!("byee byee!");
        break;
        };
    println!("continuing...");
    };
}