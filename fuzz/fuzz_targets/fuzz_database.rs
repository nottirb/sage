use honggfuzz::fuzz;
use sage_core::database::{Builder, IndexedDatabase};
use std::fs;
use std::env;

const BUCKET_SIZE: usize = 23;

fn mk_database(path: &str) -> IndexedDatabase {
    let builder = Builder {
        bucket_size: Some(BUCKET_SIZE),
        fasta: Some(path.into()),
        generate_decoys: Some(false),
        ..Default::default()
    };

    builder.make_parameters().build().unwrap()
}

fn main() {
    loop {
        fuzz!(|data: &[u8]| {
            // Generate a unique file path
            let mut path = env::temp_dir();
            path.push("fuzzing_input");

            // Write the data to the file
            if fs::write(&path, data).is_ok() {
                // If the write was successful, pass the file path to mk_database
                let _db = mk_database(path.to_str().unwrap());
            }

            // Clean up by removing the file (ignoring any errors)
            let _ = fs::remove_file(path);
        });
    }
}