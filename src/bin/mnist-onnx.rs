use tract_core::prelude::*;
use tract_onnx::prelude::*;

static DATA_6: &str = r"
0000000000000000000000000000
0000000000000000000000000000
0000000000000011100000000000
0000000000000111000000000000
0000000000001110000000000000
0000000000011100000000000000
0000000001110000000000000000
0000000111000000000000000000
0000011110000000000000000000
0000111000000000000000000000
0001111000000000000000000000
0001111001111111000000000000
0001111111100001111000000000
0001111000000000001110000000
0001111000000000000011100000
0001111000000000000001110000
0000111000000000000001110000
0000111000000000000001110000
0000011100000000000001100000
0000011100000000000111100000
0000001110000000001111000000
0000000011111111111000000000
0000000000011111000000000000
0000000000000000000000000000
0000000000000000000000000000
0000000000000000000000000000
0000000000000000000000000000
0000000000000000000000000000
";

fn as_array(date: &str) -> tract_ndarray::Array4<f32> {
    let mut v = Vec::with_capacity(28 * 28);
    for c in date.bytes() {
        match c {
            b'1' => v.push(1.0),
            b'0' => v.push(0.0),
            _ => (),
        }
    }
    tract_ndarray::Array4::from_shape_vec((1, 1, 28, 28), v).unwrap()
}

fn main() -> TractResult<()> {
    let model = tract_onnx::onnx()
        .model_for_path("mnist.onnx")?
        .with_input_fact(
            0,
            InferenceFact::dt_shape(f32::datum_type(), tvec!(1, 1, 28, 28)),
        )?
        .into_optimized()?
        .into_runnable()?;
    let input: Tensor = as_array(DATA_6).into();
    let result = model.run(tvec!(input))?;
    let mut result: Vec<(f32, usize)> = result[0]
        .to_array_view()?
        .iter()
        .cloned()
        .zip(0..)
        .collect::<Vec<_>>();
    result.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
    for (p, i) in result {
        println!("{} {:.2}%", i, p * 100.0);
    }
    Ok(())
}
