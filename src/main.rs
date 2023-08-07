use bit_vec::BitVec;
use rayon::prelude::*;
use std::{
    fs::File,
    io::{self, Read},
};

// 1m -> average ~23-24 bits per match
// 1b -> average ~33 bits per match
// 1000x multiplication ~= 1024x multiplication = 10 bits gain
// which matches the increase in the match length
// entropy works or something

fn longest_range_starting_at(
    input_bits: &BitVec,
    in_pos: usize,
    pi_bits: &BitVec,
    pi_pos: usize,
    pi_end: usize,
) -> (usize, usize, bool) {
    let start = in_pos;
    let mut beststart = 0;
    let mut bestlen = 0;
    let mut bestnot = false;
    let mut pipos = pi_pos;
    while pipos < pi_bits.len() {
        let mut innerpi = pipos;
        let mut innerin = start;
        while innerin < input_bits.len()
            && innerpi < pi_end
            && input_bits[innerin] == pi_bits[innerpi]
        {
            innerin += 1;
            innerpi += 1;
        }
        if innerin > start && (innerin - start) > bestlen {
            bestlen = innerin - start;
            beststart = pipos;
            bestnot = false;
            /*println!(
                "provisional range of len {}: {}..{} at {}..{}",
                bestlen,
                start,
                start + bestlen,
                beststart,
                beststart + bestlen
            );*/
        }
        let mut innerpi = pipos;
        let mut innerin = start;
        while innerin < input_bits.len()
            && innerpi < pi_end
            && input_bits[innerin] != pi_bits[innerpi]
        {
            innerin += 1;
            innerpi += 1;
        }
        if innerin > start && (innerin - start) > bestlen {
            bestlen = innerin - start;
            beststart = pipos;
            bestnot = true;
            /*println!(
                "provisional inv range of len {}: {}..{} at {}..{}",
                bestlen,
                start,
                start + bestlen,
                beststart,
                beststart + bestlen
            );*/
        }
        pipos += 1;
    }
    (beststart, bestlen, bestnot)
}

fn main() -> io::Result<()> {
    let mut hexbinf = File::open("pibin")?;

    let mut pibytes: Vec<u8> = vec![];
    hexbinf.read_to_end(&mut pibytes)?;
    let pibits = BitVec::from_bytes(pibytes.as_slice());
    println!("pibits.len(): {}", pibits.len());

    let mut input_file = File::open(std::env::args().take(2).last().unwrap())?;
    let mut input_bytes = vec![];
    input_file.read_to_end(&mut input_bytes)?;
    let input_bits = BitVec::from_bytes(input_bytes.as_slice());
    let mut inpos = 0;
    println!(
        "inbytes = {}, inbits = {}",
        input_bytes.len(),
        input_bits.len()
    );
    let chunk_size = pibits.len() / rayon::current_num_threads();
    let starts: Vec<usize> = (0..rayon::current_num_threads())
        .map(|i| i * chunk_size)
        .collect();
    while inpos < input_bits.len() {
        let ends = starts.par_iter().map(|start| {
            usize::min(
                start + chunk_size + (input_bits.len() - inpos) as usize - 1,
                pibits.len() as usize,
            )
        });
        let matches: Vec<(usize, usize, bool)> = starts
            .par_iter()
            .zip(ends)
            .map(|(start, end)| longest_range_starting_at(&input_bits, inpos, &pibits, *start, end))
            .collect();
        let mut bestlen = 0;
        let mut beststart = 0;
        let mut bestnot = false;
        for (start, len, invert) in matches {
            if len > bestlen {
                bestlen = len;
                beststart = start;
                bestnot = invert;
            }
        }
        assert_ne!(bestlen, 0);
        println!(
            "range (inverted? {}) of len {}: {}..{} at {}..{} ({:08x}..{:08x})",
            bestnot,
            bestlen,
            inpos,
            inpos + bestlen,
            beststart,
            beststart + bestlen,
            beststart,
            beststart + bestlen
        );
        for i in 0..bestlen {
            if bestnot {
                assert_eq!(input_bits[i + inpos], !pibits[beststart + i]);
            } else {
                assert_eq!(input_bits[i + inpos], pibits[beststart + i]);
            }
        }
        inpos += bestlen;
    }

    Ok(())
}
