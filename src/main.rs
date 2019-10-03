extern crate csv;

use std::error::Error;
//use std::io;
use std::process;
use std::env;
use std::str::FromStr;

#[derive(Clone)]
struct Bin {
	count: u32,
	total: f64,
	count_negative: u32,
	total_negative: f64,
}

impl Bin {
	fn new() -> Bin {
		Bin {
			count: 0,
			total: 0.0,
			count_negative: 0,
			total_negative: 0.0,
		}
	}
	
	fn store(&mut self, value: f64){
		if value >= 0.0 {
			self.total += value;
	    	self.count += 1;
		} else {
			self.total_negative += value;
	    	self.count_negative += 1;
		}
	    
	}

	fn render(& self) -> String {
		let mut value = 0.0;
		let mut value_negative = 0.0;

		if self.count > 0 {
			value = self.total / self.count as f64;
		}
	    if self.count_negative > 0 {
	    	value_negative = self.total_negative / self.count_negative as f64;
	    }


	    format!("{},{}", value, value_negative)
	}
	
}

struct MapRow {
	row: Vec<Bin>,
	row_number: usize,
}

impl MapRow{
	fn new(len: usize) -> MapRow {
		MapRow {
			row: vec![Bin::new(); len],
			row_number: 0,
		}
	}
	
	fn update(&mut self, index: usize, value: f64){
	    self.row[index].store(value);
	}
	
	fn erase(&mut self){
	    self.row = vec![Bin::new(); self.row.len()];
	    self.row_number += 1;
	}
	
	fn print(&mut self) {
	    for (i, bin) in self.row.iter().enumerate() {

	        println!("{},{},{}", self.row_number, i, bin.render());
	    }
	}
}

fn parse_pair<T: FromStr>(s: &str, separator: char) -> Option<(T, T)> {
    match s.find(separator) {
        None => None,
        Some(index) => {
            match (T::from_str(&s[..index]), T::from_str(&s[index + 1..])) {
                (Ok(l), Ok(r)) => Some((l, r)),
                _ => None
            }
        }
    }
}

fn example(target_file: &str, bin_size_r: usize, bin_size_c: usize) -> Result<(), Box<Error>> {
    // Build the CSV reader and iterate over each record.
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b'\t')
        .from_path(target_file)?;
        //.from_path("test.tsv")?;

    //let mut _total: f64 = 0.0;

    //let bin_size = 10;
    println!("id,col,value,value_negative");

    let columns;
    let how_many_bins;
    let mut my_row;
    let mut row_counter = 0;
    {
	    let mut iter = rdr.records();

	    if let Some(result) = iter.next() {
	        let record = result?;
	        columns = record.len() - 1;
	        how_many_bins = (columns + bin_size_c - 1)/bin_size_c;
	        my_row = MapRow::new(how_many_bins);
	    } else {
	    	panic!("No fields!")
	    }
	}

    

    



    for result in rdr.records() {
        // The iterator yields Result<StringRecord, Error>, so we check the
        // error here.
        let record = result?;


        
        for (index_row, field) in record.iter().enumerate()
        {
        	//let value = field.parse::<f64>().unwrap_or_else(|field| { println!("Value cannot be parsed into float -> {}", field); });

        	if index_row > 0{
	        		let value = field.parse::<f64>()
	        			.expect("Value cannot be parsed into float");


	        	let which_bin = (index_row - 1) / bin_size_c; //Floor of integer division

	        	//println!("place:{} bin:{}:val{}", index_row, which_bin, value);

	        	my_row.update(which_bin, value)

	        	}

        }

        row_counter += 1;

        if row_counter % bin_size_r == 0 {
        	my_row.print();
        	my_row.erase();
        }

    }

    if row_counter % bin_size_r != 0 {
    	my_row.print();
	}

    //println!("{}", _total);
    Ok(())
}

fn main() {

	let args: Vec<String> = env::args().collect();
	//let bin_size = &args[1].parse::<usize>().expect("Invalid bin size");
	let bounds = parse_pair(&args[1], ',').expect("error parsing bin dimensions");
	let rows: usize = bounds.0;
	let cols: usize = bounds.1;
	let target_file = &args[2];

    if let Err(err) = example(target_file, rows, cols) {
        println!("error running example: {}", err);
        process::exit(1);
    }
}