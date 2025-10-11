#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(deref_nullptr)]

use anyhow::{Result, anyhow};
use std::mem;

mod bindings;

pub use bindings::*;

pub fn compress(input: &[u8]) -> Result<Vec<u8>> {
    let output = vec![0u8; input.len()];
    unsafe {
        let mut stream: bz_stream = mem::zeroed();
        let result = BZ2_bzCompressInit(&mut stream as *mut _, 1, 0, 0);
        if result != BZ_OK as _ {
            return Err(anyhow!("Failed to initialize"));
        }

        stream.next_in = input.as_ptr() as *mut _;
        stream.avail_in = input.len() as _;
        stream.next_out = output.as_ptr() as *mut _;
        stream.avail_out = output.len() as _;

        let result = BZ2_bzCompress(&mut stream as *mut _, BZ_FINISH as _);
        if result != BZ_STREAM_END as _ {
            return Err(anyhow!("Failed to compress"));
        }

        let result = BZ2_bzCompressEnd(&mut stream as *mut _);
        if result != BZ_OK as _ {
            return Err(anyhow!("Failed to end compression"));
        }
    }

    Ok(output)
}

pub fn decompress(input: &[u8]) -> Result<Vec<u8>> {
    let output = vec![0u8; input.len()];
    unsafe {
        let mut stream: bz_stream = mem::zeroed();
        let result = BZ2_bzDecompressInit(&mut stream, 0, 0);
        if result != BZ_OK as _ {
            return Err(anyhow!("Failed to initialize"));
        }

        stream.next_in = input.as_ptr() as *mut _;
        stream.avail_in = input.len() as _;
        stream.next_out = output.as_ptr() as *mut _;
        stream.avail_out = output.len() as _;

        let result = BZ2_bzDecompress(&mut stream as *mut _);
        if result != BZ_STREAM_END as _ {
            return Err(anyhow!("Failed to decompress"));
        }

        let result = BZ2_bzDecompressEnd(&mut stream as *mut _);
        if result != BZ_OK as _ {
            return Err(anyhow!("Failed to end decompression"));
        }
    }
    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compression_decompression_should_work() {
        let input = include_str!("bindings.rs").as_bytes();
        let compressed = compress(input).unwrap();
        let decompressed = decompress(&compressed).unwrap();

        assert_eq!(input, &decompressed);
    }
}
