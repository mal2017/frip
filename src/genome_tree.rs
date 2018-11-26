use bio::data_structures::interval_tree::IntervalTree;
use std::collections::HashMap;
use std::str;
use core::ops::Range;
use rust_htslib::bam::record::Record;
use rust_htslib::bam::HeaderView;

#[derive(Debug)]
pub struct GenomeTree {
    pub inner: HashMap<String, IntervalTree<u32, u32>>,
}


impl GenomeTree {
    pub fn new(c: usize) -> Self {
        GenomeTree {
            inner: HashMap::with_capacity(c),
        }

    }

    pub fn from_bed_path(v: &str) -> Result<Self, GenomeTreeError> {

        // make an empty
        let mut blank = GenomeTree::new(23);

        // make the reader
        let mut regions = bio::io::bed::Reader::from_file(v).unwrap();

        // get iterator over bed regions
        let mut region_records = regions.records();

        // preallocate these
        let mut record: bio::io::bed::Record;
        let mut start: u32;
        let mut end: u32;
        let mut chrom: String;

        while let Some(r) =  region_records.next() {
            // TODO: handle circular contigs??
            match r {
                Ok(record) => {
                    chrom = record.chrom().to_string();
                    start = record.start() as u32;
                    end = record.end() as u32;
                    blank.inner.entry(chrom)
                             .and_modify(|a| a.insert(Range  {start: start, end: end},0))
                             .or_insert({ let mut a = IntervalTree::new();
                                          a.insert(Range  {start: start, end: end},0);
                                          a // return updated range
                                          });
                },
                Err(e) => {
                    continue;
                }
            };

        };
        Ok(blank)
    }

    pub fn tally_overlap(&self, chr: &str, r: &Range<u32>) -> usize {
        let contig = self.inner.get(chr).unwrap();
        contig.find(r).count()
    }

}


quick_error! {
    #[derive(Debug, Clone)]
    pub enum GenomeTreeError {
        Some {
            description("Error creating regions from your bed file.")
        }
    }
}
