use ndarray::Array2;
use rand::distributions::Distribution; // for using .sample()
use rand::thread_rng;
use rand_distr::Normal; // split from rand since 0.7
use serde::Deserialize;
use std::error::Error;
use std::fs::read_to_string;
use std::io;

#[derive(Deserialize)]
struct Config {
    centroids: [f64; 6],
    noise: f64,
    samples_per_centroid: usize,
}

fn generate_data(
    centroids: &Array2<f64>,
    points_per_centroid: usize,
    noise: f64,
) -> Result<Array2<f64>, Box<dyn Error>> {
    assert!(!centroids.is_empty(), "centroids cannot be empty.");
    assert!(noise >= 0f64, "noise must be non-negative.");

    let rows = centroids.shape()[0];
    let cols = centroids.shape()[1];
    let mut rng = thread_rng();
    let normal_rv = Normal::new(0f64, noise)?; //[2]
    let mut raw_cluster_data = Vec::with_capacity(rows * points_per_centroid * cols);

    for _ in 0..points_per_centroid {
        // [3]
        // generate points from each centroid
        for centroid in centroids.rows() {
            let mut point = Vec::with_capacity(centroids.shape()[1]);
            for feature in centroid.into_iter() {
                point.push(feature + normal_rv.sample(&mut rng));
            }
            // push point to raw_cluster_data
            raw_cluster_data.extend(point);
        }
    }
    Ok(Array2::from_shape_vec(
        (rows * points_per_centroid, cols),
        raw_cluster_data,
    )?)
}

fn main() -> Result<(), Box<dyn Error>> {
    let toml_config_str = read_to_string("config/generate.toml")?;
    let config: Config = toml::from_str(&toml_config_str)?;
    let centroids = Array2::from_shape_vec((3, 2), config.centroids.to_vec())?;
    let samples = generate_data(&centroids, config.samples_per_centroid, config.noise)?;
    let mut writer = csv::Writer::from_writer(io::stdout());
    writer.write_record(&["height", "length"])?;
    for sample in samples.rows() {
        let mut sample_iter = sample.into_iter();
        writer.serialize((sample_iter.next().unwrap(), sample_iter.next().unwrap()))?;
    }
    Ok(())
}
