use std::time::Instant;

use mnist_data::{read_images, read_labels};
use tract_onnx::prelude::*;

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
    let (samples, images) = read_images("data/t10k-images-idx3-ubyte").unwrap();
    let images = Tensor::from_shape(
        &[samples, 1, 28, 28],
        &images
            .into_iter()
            .map(|b| b as f32 / 255.0)
            .collect::<Vec<f32>>(),
    )
    .unwrap();
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
