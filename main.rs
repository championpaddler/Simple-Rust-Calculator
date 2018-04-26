use std::io;

fn main() {
    
    let mut n = String::new();

println!("#### SIMPLE CALCULATOR IN RUST ####");
println!("#### Enter Your First Number  ####");


    io::stdin()
        .read_line(&mut n)
        .expect("failed to read input.");
    let n = n.trim().parse::<i32>().expect("invalid input");

println!("#### Enter Your Second Number ####");
    let mut n1 = String::new();
    io::stdin()
        .read_line(&mut n1)
        .expect("failed to read input.");
    let n1 = n1.trim().parse::<i32>().expect("invalid input");

println!("#### Enter Your Choice####");        
let mut ch = String::new();
    io::stdin()
        .read_line(&mut ch)
        .expect("failed to read input.");
    

 
    if ch=="+\r\n"
{
println!("{:?}",n+n1);
}  
 if ch=="-\r\n" 
{
println!("{:?}",n-n1);
}    

 if ch=="*\r\n" 
{
println!("{:?}",n*n1);
}    

 if ch=="/\r\n"
{
println!("{:?}",n/n1);
}    
  

   
   
   
}
