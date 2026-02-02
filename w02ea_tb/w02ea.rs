// compute equivalent delta resistance when given Y configured resistors
// and also vice versa from Y to delta

// inputs: R_1 R_2 R_3 (each separated by a space) w/ limitations of real numbers, and
// greater than 0, but less than 10^6
// output: two lines indicating the delta of the Y: R_AB R_BC R_AC
// the Y equivalent of the delta assumption: R_A R_B R_C
// round down the values to 4 decimal places

use std::io; // from adder activity

fn main(){
	// ask for 3 resistor inputs from the user
	// R1, R2, R3
	//println!("Input:");
	
	// read the inputs then turn to a string like adder activity
	let mut str_in_resistors = String::new(); 

	// read the user input
	io::stdin()
		.read_line(&mut str_in_resistors)
		.expect("Failed to read inputs");

	//split the input into 3: R1 R2 R3
	let str_resistors_split: Vec<&str> = str_in_resistors
	.split(' ') // the split is the space
	.collect();

	if str_resistors_split.len() != 3 {
		panic!("You did not place 3 inputs");
	}
	// input checks:
	let r_1: f32 = str_resistors_split[0]
		.trim()
		.parse()
		.expect("Input is valid");
	let r_2: f32 = str_resistors_split[1]
		.trim()
		.parse()
		.expect("Input is valid");
	let r_3: f32 = str_resistors_split[2]
		.trim()
		.parse()
		.expect("Input is valid");
	// input has to be greater than 0, but less than 10^6
	// if r1 OR r2 OR r3 is wrong, then panic. rust can use logical operators
	if r_1 <= 0.0 || r_2 <= 0.0 || r_3 <= 0.0 || r_1 >= 1_000_000.0 || r_2 >= 1_000_000.0 || r_3 >= 1_000_000.0{
		panic!("Invalid inputs!!");
	}

	// need to make the resistors an array:
	let resistors_arr = [r_1, r_2, r_3]; // a is 1, b is 2, c is 3
										 // ab is 1, bc is 2, ac is 3
										 
	let mut y_to_d_vec: Vec<f32> = Vec::new(); // array for wye to delta
	let mut d_to_y_vec: Vec<f32> = Vec::new(); // array for delta to wye
	let mut num = 0.0;
	
	for i in 0..resistors_arr.len(){ // stuff to solve for wye to delta; 
		for j in i+1..resistors_arr.len(){
			num += resistors_arr[i] * resistors_arr[j] // Multiply
		}
	}
	for i in 0..resistors_arr.len(){
		y_to_d_vec.push(num/resistors_arr[i]); // expected: [R23, R13, R12]
	} y_to_d_vec = vec![y_to_d_vec[2], y_to_d_vec[0], y_to_d_vec[1]]; // REARRANGE TO [R12 R23 R13]
	
	// stuff to solve delta to wye;
	// fsr i want to make my life harder and actually automate it and not make it encoded
	let denom = r_1 + r_2 + r_3;
	for i in 0..resistors_arr.len(){
		for j in i+1..resistors_arr.len(){
			d_to_y_vec.push((resistors_arr[i] * resistors_arr[j])/denom);
		}
	} d_to_y_vec = vec![d_to_y_vec[1], d_to_y_vec[0], d_to_y_vec[2]];
	
	//println!("Output:");
	// R12 R23 R13, wye to delta
	println!("{:.4} {:.4} {:.4}", y_to_d_vec[0], y_to_d_vec[1], y_to_d_vec[2]); 
	// R1 R2 R3, delta to wye
	println!("{:.4} {:.4} {:.4}", d_to_y_vec[0], d_to_y_vec[1], d_to_y_vec[2]); 
	
}
