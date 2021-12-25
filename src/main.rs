use anyhow::Result;
use tch::{nn, nn::ModuleT, nn::OptimizerConfig, Device};

fn net(vs: &nn::Path) -> impl ModuleT {
    nn::seq_t()
        .add_fn(|xs| xs.view([-1, 1, 28, 28]))
        .add(nn::conv2d(vs, 1, 32, 5, Default::default()))
        .add_fn(|xs| xs.max_pool2d_default(2))
        .add(nn::conv2d(vs, 32, 64, 5, Default::default()))
        .add_fn(|xs| xs.max_pool2d_default(2))
        .add_fn(|xs| xs.view([-1, 1024]))
        .add(nn::linear(vs, 1024, 1024, Default::default()))
        .add_fn(|xs| xs.relu())
        .add_fn_t(|xs, train| xs.dropout(0.5, train))
        .add(nn::linear(vs, 1024, 10, Default::default()))
}

fn main() -> Result<()> {
    let m = tch::vision::mnist::load_dir("data")?;
    let vs = nn::VarStore::new(Device::cuda_if_available());
    let net = net(&vs.root());
    let mut opt = nn::Adam::default().build(&vs, 1e-4)?;
    for epoch in 1..100 {
        for (bimages, blabels) in m.train_iter(256).shuffle().to_device(vs.device()) {
            let loss = net
                .forward_t(&bimages, true)
                .cross_entropy_for_logits(&blabels);
            opt.backward_step(&loss);
        }
        let test_accuracy =
            net.batch_accuracy_for_logits(&m.test_images, &m.test_labels, vs.device(), 1024);
        println!("epoch: {:4} test acc: {:5.2}%", epoch, 100. * test_accuracy);
        vs.save("vs2.zip").unwrap();
    }
    Ok(())
}
