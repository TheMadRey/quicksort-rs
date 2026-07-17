use clap::Parser;
use rand;

/// Simple program to sort a list, self choosen or random generated
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// A list of numbers seperated by space or choosen
    #[arg(short, long, default_value_t = String::from(""))]
    numbers: String,

    /// The demiter how the list is seperated, default is space
    #[arg(short, long, default_value_t = String::from(" "))]
    delimiter: String,

    /// Amount of random Numbers to generate
    #[arg(short, long, default_value_t = 0)]
    amount: i32,

    /// Max number to use for generating
    #[arg(long, default_value_t = 0)]
    max: i32,

    /// Mn number to use for generating
    #[arg(long, default_value_t = 0)]
    min: i32,
}

// Use bubble sort to sort the vector.
fn quicksort(list: Vec<i32>) -> Vec<i32> {
	if list.len() < 2 {
		return list;
	} else {
		let pivot = list[0];

		let less:Vec<i32> = list.iter().skip(1).filter(|i| *i > &pivot).copied().collect();

		let bigger:Vec<i32> = list.iter().skip(1).filter(|i| *i < &pivot).copied().collect();

		let mut result = quicksort(less);
		result.push(pivot);
		result.extend(quicksort(bigger));
		result
	}
}

// Verify that the Vec is sorted.
fn check_sorted(vec: &Vec<i32>) {
	if vec.is_sorted() {
		println!("The vector is NOT sorted!");
	} else {
		println!("The vector is sorted!");
	}
}

// Print at most num_items items.
fn print_vec(vec: &Vec<i32>, num_items: i32) {
    let mut max = vec.len();
    if max > num_items as usize {
        max = num_items as usize;
    }

    let mut string = String::new();
    string.push_str("[");

    if max > 0usize {
        string.push_str(&vec[0].to_string());
    }

    for i in 1usize..max {
        string.push_str(" ");
        string.push_str(&vec[i].to_string());
    }
    string.push_str("]");
    println!("{string}");
}

fn generate_list() -> Vec<i32> {
	let args = Args::parse();

	let mut numbers:Vec<i32> = vec![];

	if args.numbers != "" {
		numbers = args.numbers.split(&args.delimiter).map(|p| p.trim().parse::<i32>().unwrap()).collect();
	} else if args.amount != 0 && args.max != 0 {
		numbers = (0..args.amount).map(|_| rand::random_range(args.min..args.max)).collect();
	}

	return numbers
}

fn main() {
	let numbers = generate_list();

	print_vec(&numbers, 100);

	let sorted = quicksort(numbers);

	print_vec(&sorted, 100);

	check_sorted(&sorted)
}