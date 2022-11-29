  use std::io; 

  fn main() {
    student_council_vote();

}

  fn student_council_vote() {
      
    let mut input1 = String::new();
    let mut input2 =String::new();
    let mut input3 =String::new();
    let mut _input4 = String::new();
    let mut _input5 = String::new();
    let mut _input6 = String::new();
    let mut _input7= String::new();

    for _x in 0..15 {
         println!(" Enter your level ");
         io::stdin().read_line(&mut input1).expect("Not a valid string");
         let a:f32 = input1.trim().parse().expect("Not a valid number");
    

    println!(" Enter your C.G.P.A");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let b:f32 = input2.trim().parse().expect("Not a valid number");

    println!(" Are you a class Rep");
    println!("0 for Class Rep and 1 for Not Class Rep");
    io::stdin().read_line(&mut input3).expect("Not a valid string");
    let c:i32 = input3.trim().parse().expect("Not a valid number");
    if c ==0  {
        println!("Class Rep");
    }
    else {
        println!(" Not a class rep");
    }

    if a > 100.0 && b > 4.0 && c == 0 {
        println!("You can vote ");

      println!("Name of Candidate:");
    io::stdin().read_line(&mut input3).expect("Not a valid string");
    let _d = input3.trim();

    println!("Email:" );
     io::stdin().read_line(&mut input3).expect("Not a valid string ");
    let _e = input3.trim();

    println!("Department:" );
    io::stdin().read_line(&mut input3).expect("Not a valid string");
    let _f = input3.trim();

    println!("State of Origin:" );
    io::stdin().read_line(&mut input3).expect("Not a valid string");
    let _g= input3.trim();

    
    }
    else {
        println!("Sorry, you aren't eligible to vote");
    }
  

  }




    }

   