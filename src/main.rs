extern crate csv;

use std::error::Error;
use std::io;
use std::process;
//use std::env;

#[derive(Clone)]
struct Bin {
	count: u32,
	total: f64,
}

impl Bin {
	fn new() -> Bin {
		Bin {
			count: 0,
			total: 0.0,
		}
	}
	
	fn store(&mut self, value: f64){
	    self.total += value;
	    self.count += 1;
	}

	fn render(& self) -> f64 {
	    let value = self.total / self.count as f64;
	    value
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

fn example() -> Result<(), Box<Error>> {
    // Build the CSV reader and iterate over each record.
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b'\t')
        .from_path("../simpleheat/demo/data/TCGA_TACNA_samples_sorted_ward.D2_common_tissue_type.txt")?;
        //.from_path("test.tsv")?;

    //let mut _total: f64 = 0.0;

    let bin_size = 10;

    let columns;
    {
	    let mut iter = rdr.records();

	    if let Some(result) = iter.next() {
	        let record = result?;
	        columns = record.len();
	    } else {
	    	panic!("No fields!")
	    }
	}

    let mut row_counter = 0;

    let how_many_bins = (columns + bin_size - 1)/bin_size;

    let mut my_row = MapRow::new(how_many_bins);

    println!("id,col,value");

    for result in rdr.records() {
        // The iterator yields Result<StringRecord, Error>, so we check the
        // error here.
        let record = result?;


        
        for (index_row, field) in record.iter().enumerate()
        {
        	//let value = field.parse::<f64>().unwrap_or_else(|field| { println!("Value cannot be parsed into float -> {}", field); });

        	let value = field.parse::<f64>().expect("Value cannot be parsed into float");


        	let which_bin = index_row / bin_size; //Floor of integer division

        	my_row.update(which_bin, value)

        }

        row_counter += 1;

        if row_counter % bin_size == 0 {
        	my_row.print();
        	my_row.erase();
        }

    }

    my_row.print();

    //println!("{}", _total);
    Ok(())
}

fn main() {

	//let args: Vec<String> = env::args().collect();
	//let bin_size = &args[1];

    if let Err(err) = example() {
        println!("error running example: {}", err);
        process::exit(1);
    }
}