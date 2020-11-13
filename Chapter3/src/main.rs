// Decimal			98_222
// Hex				0xff
// Octal			0o77
// Binary			0b1111_0000
// Byte (u8 only)	b'A'

fn main2() {
    let x = 5;

    let x = x + 1;

    let x = x * 2;

    println!("The value of x is: {}", x);
}

fn var_types() {
	//UTF-8 chars == 4 octets
    let _c = 'z';
    let _z = 'ℤ';
    let _heart_eyed_cat = '😻';

	// boolean
    let _t = true;
    let _f: bool = false; // with explicit type annotation

	//floats
    let _x = 2.0; // f64
    let _y: f32 = 3.0; // f32

	//tuple
    let _tup = (500, 6.4, 1);
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (_x, y, _z) = tup;
    println!("The value of y is: {}", y);

    let x: (i32, f64, u8) = (500, 6.4, 1);
    let _five_hundred = x.0;
    let _six_point_four = x.1;
    let _one = x.2;

	//array
    let _a = [1, 2, 3, 4, 5];
	let _months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
	let _a: [i32; 5] = [1, 2, 3, 4, 5];
	let a = [3; 5];//let a = [3, 3, 3, 3, 3];
    let _first = a[0];
    let _second = a[1];
    let a = [1, 2, 3, 4, 5];
    let index = 10;
    let element = a[index - 6];
    println!("The value of element is: {}", element);
}

// fn main() {
//     let x = (let y = 6); // -> error 'let thing = 24' is a statement and therefore do not return a value
// }

fn advanced_assignation(){
    let x = 5;

    let y = {
        let x = 3;
        x + 1//no semicolon at the end for it to work, otherwise the expression becomes a statement
    };

    println!("The value of y is: {}", y);
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn ifelse() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
    let condition = true;
    let number = if condition { 5 } else { 6 };
}

fn return_five() -> i32 {
    5
}

fn t(){
	let spaces = "   ";
	let _spaces = spaces.len();
}

fn data_types(){
	let _guess: u32 = "42".parse().expect("Not a number!");
}

fn another_function(x: i32, y: i8) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
	const MAX_POINTS: u32 = 100_000;
	main2();
	t();
	data_types();
	var_types();
    another_function(5, 6);
	advanced_assignation();
}
