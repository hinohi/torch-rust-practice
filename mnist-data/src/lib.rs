use std::{
    fs::File,
    io::{BufReader, Read},
    path::Path,
};

use rand::Rng;

const SIZE: usize = 28;

fn read_u32<T: Read>(reader: &mut T) -> std::io::Result<u32> {
    let mut b = [0u8; 4];
    reader.read_exact(&mut b)?;
    Ok(u32::from_be_bytes(b))
}

pub fn read_images<P: AsRef<Path>>(path: P) -> std::io::Result<(usize, Vec<u8>)> {
    let mut buf_reader = BufReader::new(File::open(path)?);
    assert_eq!(read_u32(&mut buf_reader)?, 2051);
    let samples = read_u32(&mut buf_reader)? as usize;
    let rows = read_u32(&mut buf_reader)? as usize;
    let cols = read_u32(&mut buf_reader)? as usize;
    assert_eq!(rows, SIZE);
    assert_eq!(cols, SIZE);
    let data_len = samples * rows * cols;
    let mut data = vec![0u8; data_len];
    buf_reader.read_exact(&mut data).expect("Fail to read");
    Ok((samples, data))
}

pub fn read_labels<P: AsRef<Path>>(path: P) -> std::io::Result<Vec<u8>> {
    let mut buf_reader = BufReader::new(File::open(path)?);
    assert_eq!(read_u32(&mut buf_reader)?, 2049);
    let samples = read_u32(&mut buf_reader)? as usize;
    let mut data = vec![0u8; samples];
    buf_reader.read_exact(&mut data)?;
    Ok(data)
}

pub fn gen_nan_data<R: Rng>(rng: &mut R, samples: usize) -> (Vec<u8>, Vec<u8>) {
    let n = SIZE * SIZE;
    let mut images = Vec::with_capacity(samples * n);
    for _ in 0..samples {
        let mut buf = vec![0; n];
        for _ in 0..rng.gen_range(1..n / 2) {
            buf[rng.gen_range(0..n)] = 255;
        }
        images.extend(buf.into_iter());
    }
    (images, vec![10; samples])
}
