use super::genome_tree::*;
use rust_htslib::bam;
use rust_htslib::bam::Read;
use rust_htslib::bam::HeaderView;
use std::collections::HashMap;
use bio::data_structures::interval_tree::IntervalTree;
use core::ops::Range;
use std::str;

pub fn rip(reads_file: &str, regions_file: &str, p: usize, q: u8) -> u32 {
    let mut g = GenomeTree::from_bed_path(regions_file).unwrap();

    let mut b = bam::Reader::from_path(reads_file).unwrap();
    b.set_threads(p);

    let hdrv = b.header().to_owned();

    let tid_lookup: HashMap<u32, String>  = tid_2_contig(&hdrv);

    let mut rec = bam::Record::new();


    while let Ok(r) = b.read(&mut rec) {
        match rec.mapq() > q {
            true => g.tally_overlap("chr4", &Range { start: 1000, end: 2000} ),
            false => continue,
        };

    };



    5
}


pub fn tid_2_contig(h: &HeaderView) -> HashMap<u32, String> {
	let mut dict: HashMap<u32, String> = HashMap::with_capacity(46);
	for (i,t) in h.target_names()
				  .iter().map(|a| str::from_utf8(a).unwrap())
				  .enumerate() {
		dict.insert(i as u32, t.to_owned());
	}
	dict
}
