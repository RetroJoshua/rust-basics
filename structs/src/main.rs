use std::io;
struct Process {
	pid: u32,
	at: u32,
	bt: u32,
	ct: u32,
	wt: u32,
	tat: u32,
}
fn main() {
	let mut processes:Vec<Process> = Vec::new();
	
    println!("Enter number of processes: \n");
    
    let mut input = String::new();
	
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: i32 = input.trim().parse().expect("Please enter a valid number");
    //println!("")
    for i in 0..n {
		println!("\nProcesses {}:", i+1);
		
		println!("Enter arrival time: ");
		let mut at = String::new();
		io::stdin().read_line(&mut at).expect("Failed to read");
		let at:u32 = at.trim().parse().expect("Invalid number");
		
		println!("Enter burust time: ");
		let mut bt = String::new();
		io::stdin().read_line(&mut bt).expect("Failed to read");
		let bt: u32 = bt.trim().parse().expect("Invalid number");
		
		processes.push(Process {
			pid: (i+1) as u32,
			at,
			bt,
			ct:0,
			wt:0,
			tat:0,
	});
	
	
}

fn area(rectangle: &Rectangle) -> u32 {
	rectangle.width * rectangle.height
}


