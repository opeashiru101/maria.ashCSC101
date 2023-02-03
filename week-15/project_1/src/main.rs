 use std::io;
use std::io::Read;
use std::io::Write;

fn main() {
    println!(    "Good day Sir/Madam
                  Welcome to Globacom, 
                  Please enter your role to view your data : ");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Failed to read input ");
    let role:&str = input1.trim();

    if role == "Administrator" {
        let mut file = std::fs::File::open("globacom_dbase.sql").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        file.read_to_string(&mut contents).expect("Apologies, there seems to be an error");
        println!("{}",contents);
    }

     if role == "Project Manager" {
        let mut file = std::fs::File::open("project_tb.sql").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        file.read_to_string(&mut contents).expect("Apologies, there seems to be an error");
        println!("{}",contents);
    }
  if role == "Employee" {
        let mut file = std::fs::File::open("staff_tb.sql").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        file.read_to_string(&mut contents).expect("Apologies, there seems to be an error");
        println!("{}",contents);  
    }
    if role == "Customer" {
        let mut file = std::fs::File::open("customer_tb.sql").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        file.read_to_string(&mut contents).expect("Apologies, there seems to be an error");
        println!("{}",contents);  
    }
    if role == "Vendor" {
        let mut file = std::fs::File::open("dataplan_tb.sql").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        file.read_to_string(&mut contents).expect("Apologies, there seems to be an error");
        println!("{}",contents);  
    }
}
