use honggfuzz::fuzz;
use sage_core::fasta::Fasta;

fn main() {
    loop {
        fuzz!(|data: &[u8]| {
            if let Ok(s) = std::str::from_utf8(data) {
                let _ = Fasta::parse(s.to_string(), "rev_", false);
            }
        });
    }
}