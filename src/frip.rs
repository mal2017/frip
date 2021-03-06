use super::genome_tree::*;
use rust_htslib::bam;
use rust_htslib::bam::Read;
use rust_htslib::bam::HeaderView;
use std::collections::HashMap;
use rust_htslib::bam::record::*;
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

    let mut tot: f64 = 0f64;
    let mut ip: f64 = 0f64;

    let mut chr: &str;
    let mut start: i32;
    let mut end: i32;

    while let Ok(r) = b.read(&mut rec) {
        // Check that record is
        match !rec.is_unmapped() & !rec.is_supplementary() {
            true => {
                tot = tot + 1f64;
            },
            false => continue,
        };

        match rec.mapq() > q {
            true => {
                chr = tid_lookup.get(&(rec.tid() as u32)).unwrap();
                start = rec.pos();
                end = rec.cigar().end_pos().unwrap();
                ip = ip + (g.tally_overlap(&chr, &Range { start: start as u32, end: end as u32} ) > 0) as u64 as f64
            },
            false => continue,
        };

    };

    if nofrac {
        ip
    } else {
        ip / tot
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



#[cfg(test)]
mod tests {
    use std::path::Path;
    use super::*;

    #[test]
    fn correct_rip() {
        let bampath = Path::new("test/test_1.bam").to_str().unwrap();
        let bedpath = Path::new("test/test_1.bed").to_str().unwrap();
        let rip = super::rip(bampath, bedpath, 1, 0, true);
        assert_eq!(rip, 34 as f64);
    }

    #[test]
    fn correct_frip() {
        let bampath = Path::new("test/test_1.bam").to_str().unwrap();
        let bedpath = Path::new("test/test_1.bed").to_str().unwrap();
        let frip = super::rip(bampath, bedpath, 1, 0, false);
        assert_eq!(frip, 0.85 as f64);
    }
}
