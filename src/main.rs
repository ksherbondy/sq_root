use std::io;
use std::process;
use std::f64;

fn main() {
	
	//using the vector to list powers of 2 up to 2^32, put 2^0 at start so the index of the number can be used to find 1/2 of the exp value
	let power_vec: Vec<f64> = Vec::from([0.0,2.0,4.0,8.0,16.0,32.0,64.0,128.0,256.0,512.0,
		1024.0,2048.0,4096.0,8192.0,16384.0,32768.0,65536.0, 131072.0,262144.0,524288.0, 1048576.0,
		2097152.0, 4194304.0, 8388608.0, 16777216.0, 33554432.0, 67108864.0, 134217728.0, 268435456.0, 536870912.0,
		1073741824.0, 2147483648.0, 4294967296.0]);
	
    println!("Welcome to the Square Root Finder.");
	
	let mut user_input_buffer = String::new();
	
	loop {
		println!("Enter the number you would like to find:");
		let squared_number: f64 = convert_user_input(&mut user_input_buffer);
		let highest_power: u32 = find_highest_power_of_2(squared_number, &power_vec);
		let guess: f64 = find_closest_guess(highest_power, &power_vec);
		let square_root_value: f64 = sq_root(squared_number, guess);
		println!("The square root of {} is {}.", squared_number, square_root_value);
		if !continue_program(&mut user_input_buffer) {
			break;
		}
	}
}

fn continue_program (s: &mut String) -> bool {
	println!("Would you like to continue? Press 1 to continue, press 2 to exit.");
	let user_choice = convert_user_input(s);
	if user_choice == 2.0 {
		println!("Thanks for using the Square Root finder.");
		process::exit(0);
	} 
	true
}

fn convert_user_input (s: &mut String) -> f64 {
	
	s.clear();

	loop{
		io::stdin().read_line(s).expect("Failed to read line.");
		let converted_input: f64 = match s.trim().parse() {
			Ok(num) => num,
			Err(_) => {
				println!("Invalid input. Please enter a valid number.");
				s.clear();
				continue;
			}
		};	
		return converted_input;
	}
}

fn sq_root(squared_value: f64, initial_guess: f64) -> f64 {
	/*
	Commenting this portion of the code out as it really shouldn't be checked here. This 
	check should be conducted when the user inputs a number. The user shouldn't be allowed
	to input any value below 1.
	
	if n < 0.0 {
		return f64::NAN;
	}
	if n == 0.0 {
		return 0.0;
	}
	*/
	const ROOT_ACCURACY: f64 = 0.0000001;
	let mut guess: f64 = initial_guess;
	let mut prev_guess: f64 = squared_value;
	
	while  f64::abs(guess - prev_guess) > ROOT_ACCURACY  {
		prev_guess = guess;
		guess = 0.5 * (guess + squared_value / guess);
	}
	
	guess
}

fn find_highest_power_of_2(squared_value: f64, vec: &Vec<f64>) -> u32 {
	for (i, p_val) in vec.iter().enumerate(){
		if *p_val >= squared_value {return i as u32}
	}
	panic!("Input value exceeds maxium vector size.");
}

fn find_closest_guess(power_of_2: u32, vec: &Vec<f64>) -> f64 {
	let guess_value: u32 = power_of_2 >> 1;
	return vec[guess_value as usize]
}










