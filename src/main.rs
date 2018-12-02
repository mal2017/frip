#![forbid(unsafe_code)]

extern crate clap;
extern crate rust_htslib;
extern crate bio;

fn main() {
	use clap::{Arg, App};

	let matches = App::new("frip")
                          .version("0.1.0")
                          .author("Matt Lawlor <matt.a.lawlor@gmail.com>")
                          .about("Find the FRIP score for a targeted NGS experiment. Useful for ChIPseq, ATACseq, and related assays. Note that this will only consider primary alignments.")
                          .arg(Arg::with_name("READS")
                               .help("bam/sam/cram")
                               .required(true)
                               .index(1))
                           .arg(Arg::with_name("REGIONS")
                               .help("bed file of on-target regions.")
                               .required(true)
                               .index(2))
						  .arg(Arg::with_name("MAPQ")
					  		   .help("MAPQ must be greater than OR EQUAL TO provided cutoff; default 0")
						   	   .long("mapq")
						   	   .short("m")
						   	   .takes_value(true))
                          .arg(Arg::with_name("THREADS")
                          	   .help("threads to use")
                          	   .short("p")
                          	   .long("threads")
                          	   .takes_value(true))
						  .arg(Arg::with_name("RIP")
						  	   .help("Return the number of reads in peaks. otherwise returns the fraction of reads in peaks.")
							   .short("r")
							   .long("rip"))
                          .get_matches();

    let reads_file: &str = matches.value_of("READS").unwrap();
    let regions_file: &str = matches.value_of("REGIONS").unwrap();
    let threads: usize = matches.value_of("THREADS").unwrap_or("1").parse().unwrap();
	let mapq: u8 = matches.value_of("MAPQ").unwrap_or("0").parse().unwrap();
	let nofrac: bool = matches.is_present("RIP");

    let f = frip::frip::rip(reads_file, regions_file, threads, mapq, nofrac);
    println!("{:?}", f);

}
