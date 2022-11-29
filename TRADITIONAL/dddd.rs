
use std::io;

fn main(){
	fac_pub();
}
fn fac_pub() {
	let mut input1 = String::new();
	let mut input2 = String::new();
	for _i in 0..15 {
		println!("Enter the name of program");
	io::stdin().read_line(&mut input1).expect("Not a valid string");
    let _a = input1.trim();

  println!("Enter number of papers");
	io::stdin().read_line(&mut input2).expect("Not a valid string");
    let b:i32= input2.trim().parse().expect("Not a valid number");
  
  if b < 5 && b > 3 {
  	let i:i32 = 500000;
  	println!("The incentive is:{}",i);
  }
  else if b > 5 && b < 10 {
  	let i:i32 = 800000;
  	println!("The incentive is:{}",i);
  }
  else if b > 10 {
  	let i:i32 = 1000000;
  	println!("The incentive is:{}",i);
  }
  else if b < 3{
  	let i:i32 = 10000;
  	println!("The incentive is:{}",i);
  }
}
	}
	