use tabled::Tabled;

#[derive(Debug, Clone, PartialOrd, PartialEq)]

pub struct Sequence {
    pub header: String,
    pub sequence: String,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]

pub struct Search {
    pub name: String,
    pub sequence: String,
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]

pub struct Motifcatcher {
    pub name: String,
    pub sequence: String,
    pub start: usize,
    pub end: usize,
    pub upstream: String,
    pub downstream: String,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]

pub struct Filesnatch {
    pub name: String,
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]

pub struct Alignment {
    pub alignmentregion: String,
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]

pub struct Code {
    pub parent_id: String,
    pub cds_start: String,
    pub cds_end: String,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct Readlength {
    pub five: usize,
    pub hundred: usize,
    pub hundredfifty: usize,
    pub twohundred: usize,
    pub morethan: usize,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]

pub struct Pafanalyzer {
    pub query: String,
    pub query_length: usize,
    pub query_start: usize,
    pub query_end: usize,
    pub strand: String,
    pub target: String,
    pub target_length: usize,
    pub target_start: usize,
    pub target_end: usize,
    pub residue_matches: usize,
    pub alignment_length: usize,
    pub quality: usize,
}

#[derive(Debug, Tabled)]
pub struct Tableinformation {
    pub information: &'static str,
    pub query: usize,
    pub residue: usize,
    pub alignment: usize,
    pub target: usize,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]

pub struct Multiplepattern<'a> {
    pub name: &'a String,
    pub collectionvec: &'a Vec<String>,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]

pub struct PAFAnnotate {
    pub target: String,
    pub target_gff: String,
    pub target_start: usize,
    pub target_end: usize,
    pub target_gff_start: usize,
    pub target_gff_end: usize,
    pub annotategene: String,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct QuerypafView {
    pub query: String,
    pub length: usize,
    pub start: usize,
    pub end: usize,
    pub strand: String,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct ReferencepafView {
    pub reference: String,
    pub length: usize,
    pub start: usize,
    pub end: usize,
    pub residuematch: usize,
    pub alignmentblock: usize,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct QueryComparativeHolder {
    pub query1: String,
    pub length1: usize,
    pub start1: usize,
    pub end1: usize,
    pub strand1: String,
    pub query2: String,
    pub length2: usize,
    pub start2: usize,
    pub end2: usize,
    pub strand2: String,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct RefComparativeHolder {
    pub reference1: String,
    pub length1: usize,
    pub start1: usize,
    pub end1: usize,
    pub reference2: String,
    pub length2: usize,
    pub start2: usize,
    pub end2: usize,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct ResidueAlignment {
    pub residuematch1: usize,
    pub residuematch2: usize,
    pub alignmentblock1: usize,
    pub alignmentblock2: usize,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct AlignmentGFF {
    pub id: String,
    pub genomefeature: String,
    pub start: usize,
    pub end: usize,
    pub strand: String,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Positive {
    pub id: String,
    pub genomefeature: String,
    pub start: usize,
    pub end: usize,
    pub strand: String,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Negative {
    pub id: String,
    pub genomefeature: String,
    pub start: usize,
    pub end: usize,
    pub strand: String,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Sequenceadd {
    pub id: String,
    pub sequence: String,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct CaptureSeq {
    pub id: String,
    pub seq: String,
    pub strand: String,
}
#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct MRNA {
    pub id: String,
    pub seq: String,
    pub strand: String,
}
#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct CDS {
    pub id: String,
    pub seq: String,
    pub strand: String,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct POSITIVE {
    pub id: String,
    pub seq: String,
    pub strand: String,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct NEGATIVE {
    pub id: String,
    pub seq: String,
    pub strand: String,
}
