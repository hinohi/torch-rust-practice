use anyhow::{Context, Result};
use mnist_data::{gen_nan_data, read_images, read_labels};
use rand_pcg::Mcg128Xsl64;
use tch::{
    kind::Kind, nn, nn::ModuleT, nn::OptimizerConfig, vision::dataset::Dataset, Device, Tensor,
};

fn net(vs: &nn::Path) -> impl ModuleT {
    nn::seq_t()
        .add_fn(|xs| xs.view([-1, 1, 28, 28]))
        .add(nn::conv2d(vs / "conv2d_1", 1, 32, 5, Default::default()))
        .add_fn(|xs| xs.max_pool2d_default(2))
        .add(nn::conv2d(vs / "conv2d_2", 32, 64, 5, Default::default()))
        .add_fn(|xs| xs.max_pool2d_default(2))
        .add_fn(|xs| xs.view([-1, 1024]))
        .add(nn::linear(vs / "linear_1", 1024, 1024, Default::default()))
        .add_fn(|xs| xs.relu())
        .add_fn_t(|xs, train| xs.dropout(0.5, train))
        .add(nn::linear(vs / "linear_2", 1024, 11, Default::default()))
}

fn load_image_label(
    rng: &mut Mcg128Xsl64,
    image_path: &str,
    label_path: &str,
) -> std::io::Result<(Tensor, Tensor)> {
    let (samples, mut images) = read_images(image_path)?;
    let mut labels = read_labels(label_path)?;
    let (nan_images, nan_labels) = gen_nan_data(rng, samples / 2);
    images.extend(nan_images.into_iter());
    labels.extend(nan_labels.into_iter());
    let samples = samples + samples / 2;
    let images = Tensor::of_slice(&images)
        .view((samples as i64, 28 * 28))
        .to_kind(Kind::Float);
    let labels = Tensor::of_slice(&labels).to_kind(Kind::Int64);
    Ok((images / 255.0, labels))
}

fn load_data() -> std::io::Result<Dataset> {
    let mut rng = Mcg128Xsl64::new(1);
    let (train_images, train_labels) = load_image_label(
        &mut rng,
        "data/train-images-idx3-ubyte",
        "data/train-labels-idx1-ubyte",
    )?;
    let (test_images, test_labels) = load_image_label(
        &mut rng,
        "data/t10k-images-idx3-ubyte",
        "data/t10k-labels-idx1-ubyte",
    )?;
    Ok(Dataset {
        train_images,
        train_labels,
        test_images,
        test_labels,
        labels: 11,
    })
}

fn main() -> Result<()> {
    let m = load_data().context("Fail to read data")?;
    let vs = nn::VarStore::new(Device::cuda_if_available());
    let net = net(&vs.root());
    let mut opt = nn::Adam::default().build(&vs, 1e-4)?;
    for epoch in 1..100 {
        for (images, labels) in m.train_iter(256).shuffle().to_device(vs.device()) {
            let loss = net
                .forward_t(&images, true)
                .cross_entropy_for_logits(&labels);
            opt.backward_step(&loss);
        }
        let test_accuracy =
            net.batch_accuracy_for_logits(&m.test_images, &m.test_labels, vs.device(), 1024);
        println!("epoch: {:4} test acc: {:5.2}%", epoch, 100. * test_accuracy);
        vs.save("vs.zip").unwrap();
    }
    Ok(())
}
