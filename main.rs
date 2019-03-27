use std::io;


fn main() {
    let mass_base_pair: f64 = 617.96;
    let mass_base: f64 = mass_base_pair / 2_f64;
    let avogadro_constant: f64 = 6.02214085774e23;

    loop {
    	println!("Please choose dsDNA(1) or ssDNA(2)! Input 3 or more if you're want to finish.");

    	let mut start_number = String::new();
    	io::stdin().read_line(&mut start_number)
        	.expect("Failed to read line");
	    let start_number: u64 = start_number.trim().parse()
    	    .expect("Please type a number!");

    	if start_number > 2 {
    		println!("Thank you for using this programm!");
    		break;

    	} else {
    		println!("What is the sequence lenght (bp)?");
    		let mut lenght_of_seq = String::new();
    		io::stdin().read_line(&mut lenght_of_seq).expect("Failed to read line");
    		let lenght_of_seq: f64 = lenght_of_seq.trim().parse().expect("Failed to read line");

    		println!("What is the weight (ng)?");
    		let mut weight_of_seq = String::new();
    		io::stdin().read_line(&mut weight_of_seq).expect("Failed to read line");
    		let weight_of_seq: f64 = weight_of_seq.trim().parse().expect("Failed to read line");

    		println!("How many DNA copies do you want in 1 µl?");
    		let mut wishes_copy_seq = String::new();
    		io::stdin().read_line(&mut wishes_copy_seq).expect("Failed to read line");
    		let wishes_copy_seq: f64 = wishes_copy_seq.trim().parse().expect("Failed to read line");

    		if start_number == 1 {
    			let molecular_weight = lenght_of_seq * mass_base_pair + 36.04;

    			let mol = ((weight_of_seq * 1e-9_f64) / molecular_weight) * 1e12_f64;

       			let copy_number = mol * 1e-12_f64 * avogadro_constant;
    			println!("DNA copy number: {} into {}ng",  copy_number as u64, weight_of_seq as u64);

    			let wishes_weight = wishes_copy_seq / avogadro_constant * molecular_weight * 1e9_f64;
    			println!("Weight of {} DNA copies it is {} ng", wishes_copy_seq as u64, wishes_weight);

    			let mut water_volume = weight_of_seq/wishes_weight;

    			println!("You need to use {} µl of water to dissolve DNA", water_volume as u64);
    			println!("");
    			let mut counter = 1_u8;
    			while water_volume > 1000_f64 {
    				water_volume = water_volume / 1000_f64;
    				if counter == 1 {
    					println!("{}. Add 1000 µl of water to dissolve DNA", counter);
    				} else {
    					println!("{}. Add 999 µl of water into 1 µl DNA", counter);
    				}
    				counter = counter +1;
    			}
    			println!("{}. Add {} µl of water into 1 µl DNA", counter, water_volume as u64 - 1);

    		} else {
    			let molecular_weight = lenght_of_seq * mass_base + 18.02;

    			let mol = ((weight_of_seq * 1e-9_f64) / molecular_weight) * 1e12_f64;

       			let copy_number = mol * 1e-12_f64 * avogadro_constant;
    			println!("DNA copy number: {} into {}ng.",  copy_number as u64, weight_of_seq as u64);

    			let wishes_weight = wishes_copy_seq / avogadro_constant * molecular_weight * 1e9_f64;
    			println!("Weight of {} DNA copies it is {} ng", wishes_copy_seq as u64, wishes_weight);
    			let mut water_volume = weight_of_seq/wishes_weight;

    			println!("You need to use {} µl of water to dissolve DNA.", water_volume as u64);
    			println!("");
    			let mut counter = 1_u8;
    			while water_volume > 1000_f64 {
    				water_volume = water_volume / 1000_f64;
    				if counter == 1 {
    					println!("{}. Add 1000 µl of water to dissolve DNA", counter);
    				} else {
    					println!("{}. Add 999 µl of water into 1 µl DNA", counter);
    				}
    				counter = counter +1;
    			}
    			println!("{}. Add {} µl of water into 1 µl DNA", counter, water_volume as u64 - 1);


    		}

    	}








   	}
}

