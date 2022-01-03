use anyhow::Context;
use tract_onnx::{
    prelude::*,
    tract_core::model::{TypedModel, TypedRunnableModel},
};

pub struct NN {
    model: TypedRunnableModel<TypedModel>,
}

impl NN {
    pub fn new() -> NN {
        let data = include_bytes!("../../mnist.onnx");
        let model = onnx()
            .model_for_read(&mut data.as_slice())
            .unwrap()
            .with_input_fact(
                0,
                InferenceFact::dt_shape(f32::datum_type(), [1, 1, 28, 28]),
            )
            .unwrap()
            .into_optimized()
            .unwrap()
            .into_runnable()
            .unwrap();
        NN { model }
    }

    pub fn run(&self, data: &[f32]) -> anyhow::Result<[f32; 10]> {
        let tensor = Tensor::from_shape(&[1, 1, 28, 28], data)?;
        let result = self.model.run(tvec![tensor]).context("Fail to run model")?;
        let mut prob = [0.0; 10];
        for (d, s) in prob
            .iter_mut()
            .zip(result[0].to_array_view::<f32>()?.iter())
        {
            *d = *s;
        }
        Ok(prob)
    }
}
