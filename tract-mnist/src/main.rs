use std::{
    fs::File,
    io::{BufReader, Read},
    path::Path,
    time::Instant,
};

use tract_onnx::prelude::*;

fn read_u32<T: Read>(reader: &mut T) -> std::io::Result<u32> {
    let mut b = [0u8; 4];
    reader.read_exact(&mut b)?;
    Ok(u32::from_be_bytes(b))
}

fn read_images<P: AsRef<Path>>(path: P) -> std::io::Result<Tensor> {
    let mut buf_reader = BufReader::new(File::open(path)?);
    assert_eq!(read_u32(&mut buf_reader)?, 2051);
    let samples = read_u32(&mut buf_reader)? as usize;
    let rows = read_u32(&mut buf_reader)? as usize;
    let cols = read_u32(&mut buf_reader)? as usize;
    let data_len = samples * rows * cols;
    let mut data = vec![0u8; data_len];
    buf_reader.read_exact(&mut data).expect("Fail to read");
    let arr = Tensor::from_shape(
        &[samples, 1, 28, 28],
        &data
            .into_iter()
            .map(|b| b as f32 / 255.0)
            .collect::<Vec<f32>>(),
    )
    .unwrap();
    Ok(arr)
}

fn read_labels<P: AsRef<Path>>(path: P) -> std::io::Result<Vec<u8>> {
    let mut buf_reader = BufReader::new(File::open(path)?);
    assert_eq!(read_u32(&mut buf_reader)?, 2049);
    let samples = read_u32(&mut buf_reader)? as usize;
    let mut data = vec![0u8; samples];
    buf_reader.read_exact(&mut data)?;
    Ok(data)
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn main() -> TractResult<()> {
    let start = Instant::now();
    let model = tract_onnx::onnx()
        .model_for_path("mnist.onnx")?
        .with_input_fact(
            0,
            InferenceFact::dt_shape(f32::datum_type(), [1, 1, 28, 28]),
        )?
        .into_optimized()?
        .into_runnable()?;
    println!("load onnx model: {:.3}sec", start.elapsed().as_secs_f64());
    print_type_of(&model);

    let start = Instant::now();
    let images = read_images("data/t10k-images-idx3-ubyte").unwrap();
    let labels = read_labels("data/t10k-labels-idx1-ubyte").unwrap();
    println!(
        "load mnist test data: {:.3}sec",
        start.elapsed().as_secs_f64()
    );

    let start = Instant::now();
    let mut samples = 0;
    let mut correct = 0;
    for i in 0..images.shape()[0] {
        let image = images.slice(0, i, i + 1).unwrap();
        let result = model.run(tvec!(image)).unwrap();
        let (_, label) = result[0]
            .to_array_view::<f32>()?
            .iter()
            .cloned()
            .zip(0..)
            .max_by(|a, b| a.0.partial_cmp(&b.0).unwrap())
            .unwrap();
        if label == labels[i] {
            correct += 1;
        }
        samples += 1;
    }
    println!(
        "run: samples={} correct={} correct_rate={:.2}% {:.3}sec",
        samples,
        correct,
        correct as f64 / samples as f64 * 100.0,
        start.elapsed().as_secs_f64()
    );
    Ok(())
}
