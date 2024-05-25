use std::io::Write;

#[allow(dead_code)]
mod corpus {
    include!("src/corpus.rs");
}

fn main() {
    // load default corpus
    let default = corpus::load_corpus("cpu_rec_corpus/*.corpus").unwrap();
    println!("cargo:rerun-if-changed=cpu_rec_corpus");

    // serialize to bytes
    let bytes = postcard::to_stdvec(&default).unwrap();
    // generate file path
    let compressed_file_pathbuf =
        std::path::Path::new(std::env::var("OUT_DIR").unwrap().as_str()).join("default.pc");
    let compressed_file_path = compressed_file_pathbuf.as_path();

    // open file
    let compressed_file = std::fs::File::create(compressed_file_path).unwrap();

    // write compressed to file
    let mut compressor = lz4_flex::frame::FrameEncoder::new(compressed_file);
    compressor.write_all(&bytes).unwrap();
    compressor.finish().unwrap();
}
