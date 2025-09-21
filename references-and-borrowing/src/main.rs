fn main() {
    //1.Basic ownership
    let mut message = String::from("Hello");
    println!("Original: {}",message);
    
    //2. Immutable borrowing - multiple readers allowed
    
    let ref1 = &message;
    let ref2 = &message;
    println!("Reading via ref1:{}", ref1);
    println!("Reading via ref2:{}",ref2);
    println!("Original still workes,{}",message);
    
    //3. Mutable borrowing - exclusive access
    
    {
		let ref_mut = &mut message;
		ref_mut.push_str(", world!");
		println!("Modified mutable {}",ref_mut);
		//ref_mut goes out of scope here
	}
	
	//4. Back to immutable borrowing after mutable borrow ends
	let ref3 = &message;
	println!("FInal result: {}",ref3);
	
	//5. Function that borrows immutably
	let length = calculate_length(&message);
	println!("Length: {}", length);
	
	//6. Function that borrows mutably
	add_exclamation(&mut message);
	println!("After adding exclamanation: {}", message);
}

// Function that takes an immutable reference
fn calculate_length(s: &String) -> usize{
	s.len()
	//s goes out of scope
	//nothing happens to the actual data
}

fn add_exclamation(s: &mut String){
	s.push('!');
}
