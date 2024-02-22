use linfa::traits::Fit;
use linfa::traits::Predict;
use linfa::DatasetBase;
use linfa_clustering::KMeans;
use linfa_nn::distance::L2Dist;
use ndarray::Array1;
use ndarray::Array2;
use rand::thread_rng;
use std::error::Error;
use std::io;

const CLUSTER_COUNT: usize = 3;

fn main() -> Result<(), Box<dyn Error>> {
    let samples = read_data_from_stdin()?; // To be added
    let training_data = DatasetBase::from(samples);
    let rng = thread_rng();
    let model = KMeans::params_with(CLUSTER_COUNT, rng, L2Dist)
        .max_n_iterations(200)
        .tolerance(1e-5)
        .fit(&training_data)?;
    // Assign each point to a cluser using the set of
    // centroids fount using `fit`
    let dataset = model.predict(training_data);
    let DatasetBase {
        records, targets, ..
    } = dataset;
    export_result_to_stdout(records, targets)?; // To be added
    Ok(())
}

fn read_data_from_stdin() -> Result<Array2<f64>, Box<dyn Error>> {
    let mut points: Vec<f64> = Vec::new();
    let mut reader = csv::Reader::from_reader(io::stdin());
    for result in reader.records() {
        let record = result?;
        points.push(record[0].parse()?);
        points.push(record[1].parse()?);
    }
    let rows = points.len() / 2;
    let cols = 2;
    Ok(Array2::from_shape_vec((rows, cols), points)?)
}

fn export_result_to_stdout(
    points: Array2<f64>,
    classes: Array1<usize>,
) -> Result<(), Box<dyn Error>> {
    let mut writer = csv::Writer::from_writer(io::stdout());
    writer.write_record(["height", "length", "class"])?;
    for (point, class) in points.rows().into_iter().zip(classes.into_iter()) {
        let mut row_iter = point.into_iter();
        writer.serialize((row_iter.next().unwrap(), row_iter.next().unwrap(), class))?;
    }
    Ok(())
}
