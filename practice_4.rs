   fn main()  {
let p:f64 = 210000.0;
let r:f64 = 5.0;
let n:f64 = 3.0;

let a  = p * (1.0 + (r / 100.0)) * n;
println!("Depreciation is {}", a);

}