use plotters::prelude::*;
use std::error::Error;
use std::io;

fn main() -> Result<(), Box<dyn Error>> {
    let mut x: [Vec<f64>; 3] = [Vec::new(), Vec::new(), Vec::new()];
    let mut y: [Vec<f64>; 3] = [Vec::new(), Vec::new(), Vec::new()];
    let mut reader = csv::Reader::from_reader(io::stdin());
    for result in reader.records() {
        let record = result?;
        let class: usize = record[2].parse()?;
        x[class].push(record[0].parse()?);
        y[class].push(record[1].parse()?);
    }
    let root_drawing_area =
        BitMapBackend::new("k-means-result-plot.png", (900, 600)).into_drawing_area();
    root_drawing_area.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root_drawing_area)
        .caption("Cat body measurements", ("sans-serif", 30))
        .x_label_area_size(30)
        .y_label_area_size(40)
        .build_cartesian_2d(15.0..45.0, 30.0..55.0)?;
    chart
        .configure_mesh()
        .x_desc("height (cm)")
        .y_desc("width (cm)")
        .disable_mesh()
        .draw()?;
    chart
        .draw_series(x[0].iter().zip(y[0].iter()).map(|point| {
            Cross::new(
                (*point.0, *point.1),
                3,
                Into::<ShapeStyle>::into(BLUE).stroke_width(2),
            )
        }))?
        .label("Cat breed 1")
        .legend(|(x, y)| Cross::new((x, y), 3, Into::<ShapeStyle>::into(BLUE).stroke_width(2)));
    chart
        .draw_series(x[1].iter().zip(y[1].iter()).map(|point| {
            TriangleMarker::new(
                (*point.0, *point.1),
                5,
                Into::<ShapeStyle>::into(GREEN).stroke_width(2),
            )
        }))?
        .label("Cat breed 2")
        .legend(|(x, y)| {
            TriangleMarker::new((x, y), 5, Into::<ShapeStyle>::into(GREEN).stroke_width(2))
        });
    chart
        .draw_series(x[2].iter().zip(y[2].iter()).map(|point| {
            Circle::new(
                (*point.0, *point.1),
                3,
                Into::<ShapeStyle>::into(RED).stroke_width(2),
            )
        }))?
        .label("Cat breed 3")
        .legend(|(x, y)| Circle::new((x, y), 3, Into::<ShapeStyle>::into(RED).stroke_width(2)));

    chart
        .configure_series_labels()
        .position(SeriesLabelPosition::LowerRight)
        .background_style(WHITE.mix(0.8))
        .border_style(BLACK)
        .draw()?;
    Ok(())
}
