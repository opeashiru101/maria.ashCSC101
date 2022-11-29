
// Question 1
fn main() {
	fn geopo_merger(){}

}
fn geopo_merger() {

	let f1:&str= "Aigbogun Alamba Daudu";
	let f2:&str="Murtala Afeez Bendu";
    let f3:&str="Okorocha Callistus Ogbona";
    let f4:&str= "Adewale Jimoh Akanbi";
    let f5:&str="Osazuwa Faith Etieye" ;


let name_commisioner:[&str;5] = [" Aigbogun Alamba Daudu", "Murtala Afeez Bendu ","Okorocha Callistus Ogbona"
	,"Adewale Jimoh Akanbi ","Osazuwa Faith Etieye"];

	let ministry1=" Internal Affairs";
	let ministry2 = "Justice";
	let ministry3="Defense";
	let ministry4= "Power and Steel"
	let ministry5 = "Petroleum";
let ministry:[&str;5]  = ["Internal Affairs", "Justice", "Defense", "Power & Steel" , " Petroleum"];
let geopolitical_zone:[&str;5] = [ "South West", "South South", "South West ", "South East"];

	println!(" Enter name of comissioner:");
	io::stdin().read_line(&mut input1).expect("Not a valid string");
	let input= String::new()

if input == f1{
	println!(" The Minister in charge and zone: {}",ministry1,geopolitical_zone1);
  
}
else if input ==f2{
println!("The Minister in Charge and zone is {}",ministry2,geopolitical_zone2 );
}
else if input == f3{
	println!("The Minister in charge and zone is {}",ministry3,geopolitical_zone3 );
}
else if input == f4 {
	println!("The Minister in charge and zone is {}",ministry4,geopolitical_zone4 );
}
else if input == f5{
	println!("The Minister in charge and zone is {}",ministry5,geopolitical_zone5);
}

}



// Question 2
use std::io;
 fn main() {
 pub_service();	
}
fn pub_service(){
	let mut input1 = String::new();
	let mut input2 =String::new();
