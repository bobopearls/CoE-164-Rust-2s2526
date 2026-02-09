use std::io;

fn main() {
	let mut str_in = String::new();

	// Read T // number of testcases
	str_in.clear();
	io::stdin()
		.read_line(&mut str_in)
		.expect("Failed to read T");
	let t: usize = str_in.trim()
		.parse()
		.expect("T must be an integer");

	
	for case_idx in 1..=t {
		// Read S the string depending on the number of test cases
		// TODO: Edit this function to parse the input and
		//       process the tasks.
		str_in.clear(); // should actually be here instead
		io::stdin()
			.read_line(&mut str_in)
			.expect("Failed to Read S");
		let s = str_in.trim(); // trim the string into its individual characters

		let normalized_s = normalize_code(s); // turn the string to all digits
		let (count, res) = amplify(&normalized_s);
		let res_letter = digi_to_letter(&res); // make it actually letter output and not number
		println!("Testcase#{}: {} {}", case_idx, count, res_letter);

	}
}

// helper codes are good already
fn normalize_code(digi_code: &str) -> String {
    // TODO: Edit this function to normalize a digit or letter code 
    //       string to a digit string
    let mut digi_code_str = String::new(); // output
    for d in digi_code.chars(){
		if d >= 'A' && d <= 'J' {
			digi_code_str.push( ( (d as u8 -b'A') + b'0') as char );
		}
		else if d >= '0' && d <= '9'{
			digi_code_str.push(d); // keep as is
		}
    	/*
    	EQUIVALENT CODE THAT IS WAY TOO LONG. i was lazy to understand the b'A' stuff at first
    	if d == 'A'{
    		digi_code_str.push('0');
    	}
    	else if d == 'B'{
    		digi_code_str.push('1');
    	}
    	else if d == 'C'{
    		digi_code_str.push('2');
    	}
    	else if d == 'D'{
    		digi_code_str.push('3');
    	}
    	else if d == 'E'{
    		digi_code_str.push('4');
    	}
    	else if d == 'F'{
    		digi_code_str.push('5');
    	}
    	else if d == 'G'{
    		digi_code_str.push('6');
    	}
    	else if d == 'H'{
    		digi_code_str.push('7');
    	}
    	else if d == 'I'{
    		digi_code_str.push('8');
    	}
    	else if d == 'J'{
    		digi_code_str.push('9');
    	}
    	else if d>= '0' && d<= '9' { // if it is a digit, then keep it as is
    		digi_code_str.push(d);
    	}  // no need to use push_str since it returns a string already
    	else{
			// wala
		}

    	 */
    }
    
    digi_code_str
}

fn amplify(digits: &str) -> (u32, String) {
    // TODO: Edit this function to repeatedly apply the reverse-and-add 
    //       process until a palindrome is reached (i used helper functions)
	let mut count: u32 = 0;
	let mut curr_dig = digits.to_string();

	while !palindrome_checker(&curr_dig) {
		let reverse = reverse_digits_str(&curr_dig); // reverse string using the function
		curr_dig = add_digi_str(&curr_dig, &reverse);
		count += 1;
	}
	(count, curr_dig)

}

// Optional: You may add more helper functions for smaller tasks such as:
//           checking if the string is a palindrome, reversing the string,
//           adding two digit-only strings, converting digit-only to letter strings
fn reverse_digits_str(s: &str) -> String {
	s.chars().rev().collect() // reverse then collect the string
}

fn palindrome_checker(s: &str) -> bool{
	// returns a true or false to see if the thing is a palindrome
	s.chars().eq( s.chars().rev() ) // reverse the thing to see if it is a palindrome
}

fn add_digi_str(a: &str, b: &str) -> String {
	// two digit only strings, assumes that the string inputs have already been converted to digits
	// addition of strings, not integers (entire integer might be too large, so break it down per digit)
	// a + b then keep track of the carry overs, then move to the next digit place
	let mut carry_over: u32 = 0;
	let mut res = String::new();

	// split into individual digits then reverse a and b
	// we reverse it because we want to start with the ones place, so make it the 0th dig in the list
	let mut rev_a = a.chars().rev();
	let mut rev_b = b.chars().rev();

	loop{
		// start from the rightmost digit which we already reversed
		let next_a = rev_a.next();
		let next_b = rev_b.next();

		// if no more digits AND no more carry over, then the addition is complete
		if next_a.is_none() && next_b.is_none() && carry_over == 0{
			break;
		}

		let mut char_to_dig_a: u32 = 0;
		let mut char_to_dig_b: u32 = 0; // convert the characters to digits after the checking
		if let Some(char_a) = next_a{
			char_to_dig_a = char_a.to_digit(10).unwrap();
		}
		if let Some(char_b) = next_b{
			char_to_dig_b = char_b.to_digit(10).unwrap();
		}

		let res_sum = char_to_dig_a + char_to_dig_b + carry_over;
		carry_over = res_sum / 10;
		let digit = res_sum % 10;

		res.push( (b'0' + digit as u8) as char);
	}
	res.chars().rev().collect() // reverse the number back to the original / intended
}

fn digi_to_letter(s: &str) -> String{ // convert it back to letter
									  // (palindrome works but its digit so i need to change it back)
	let mut out_dig_to_letter = String::new();
	for digi in s.chars(){
		let d = digi as u8 - b'0'; // if we have d = 3,
								   // then b'A' + d is = 65 + 3 = 68 which reverts back to letter D
		out_dig_to_letter.push( (b'A' + d) as char);
	}
	out_dig_to_letter
}
