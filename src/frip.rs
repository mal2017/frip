use super::genome_tree::*;
use rust_htslib::bam;
use rust_htslib::bam::Read;
use rust_htslib::bam::HeaderView;
use std::collections::HashMap;
use bio::data_structures::interval_tree::IntervalTree;
use core::ops::Range;
use std::str;

pub fn rip(reads_file: &str, regions_file: &str, p: usize, q: u8, nofrac: bool) -> f64 {
    let mut g = GenomeTree::from_bed_path(regions_file).unwrap();

    let mut b = bam::Reader::from_path(reads_file).unwrap();
    b.set_threads(p);

    let hdrv = b.header().to_owned();

    let tid_lookup: HashMap<u32, String>  = tid_2_contig(&hdrv);

    let mut rec = bam::Record::new();

    let mut tot: u32 = 0;
    let mut ip: u32 = 0;

    let mut chr: &str;
    let mut start: i32;
    let mut end: i32;

    while let Ok(r) = b.read(&mut rec) {
        // TODO check for primary alignment
        tot = tot + 1;
        match rec.mapq() > q {
            true => {
                chr = tid_lookup.get(&(rec.tid() as u32)).unwrap();
                if rec.is_reverse() {
                    end = rec.pos();
                    start = end - (rec.seq().len() as i32);
                } else {
                    start = rec.pos();
                    end = start + (rec.seq().len() as i32);
                };
                ip = ip + (g.tally_overlap(&chr, &Range { start: start as u32, end: end as u32} ) > 0) as u32
            },
            false => continue,
        };

    };

    if nofrac {
        ip as f64
    } else {
        ip as f64 / tot as f64
    }
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
