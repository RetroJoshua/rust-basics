fn main() {
    println!("===OWNERSHIP DEMONESTRATION===\n");
    
    //1.Basic ownership and scope
    println!("1. Basic Ownership & Scope:");
    {
		let s1 = String::from("Hello");
		println!(" s1={}",s1);
		//s1 is valid here
	}// s1 goes out of scope and is dropped here
	//println!("{}",s1); //This would cause a compile error!
	println!();
	//2. Move semantics
	println!("2.Move Semantics:");
	let s2 = String::from("World");
	println!(" Before move: s2 = {}",s2);
	
	let s3 = s2; // s2 is mpved to s3, s2 is no longer valid
	println!(" After move: s3 = {}",s3);
	//println!("{}",s2); //This would cause compilation error!
	
	println!();
	
	//3. copy vs move
	println!("3. Copy vs Move:");
	let x= 42;  //i32 implements copy
	let y= x;   //x is copied, not moved
	println!("	x = {}, y = {} (both valid!)",x,y);
	
	let str1 = String::from("Rust");
	let str2 = str1; //string is moved not copied
	println!(" str2 = {} (str1 is no longer valid)", str2);
	
	println!();
	
	//4.clone or Deep copy
	println!("4. Clone or Deep Copy");
	let original = String::from("Clone me!");
	let cloned = original.clone(); //explicit deep copy
	println!(" original = {} ", original);
	println!(" cloned = {} ", cloned);
	
	println!();
	
	//5. Functions and ownership
	println!("5. Functions and Ownership");
	let message = String::from("Function test");
	println!(" Before function call: {}", message);
	
	takes_ownership(message); //message moved into function
	//println!("{}",message); //this would cause a compile error!
	
	let number = 100;
	println!(" Before function call: {}", number);
	makes_copy(number);
	println!(" After function call: {}", number);
	
	println!();
	
	//6.Returning Ownership
	println!("6. Returning Ownership");
	let returned_string = gives_ownership();
	println!(" Received: {}", returned_string);
	
	let input = String::from("Take and return");
	let output = takes_and_gives_back(input);
	println!(" Returned: {}", output);
	//println!("{}",input);//this would cause a compile time error!
}

fn takes_ownership(some_string: String){
	println!(" Inside function:{}", some_string);
	//some string goes out of scope and is dropped
}

fn makes_copy(some_integer:i32){
	println!(" Inside function:{}",some_integer);
	//some_integer goes out of scope but its just a copy
}

fn gives_ownership()->String {
	let some_string = String::from("I'm yours now!");
	some_string // Return moves ownership back to caller
}

fn takes_and_gives_back(a_string: String)-> String {
	a_string //Return moves ownership back to caller
}
