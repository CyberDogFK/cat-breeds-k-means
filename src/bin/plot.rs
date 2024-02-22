use plotters::prelude::*;
use std::error::Error;
use std::io;

fn main() -> Result<(), Box<dyn Error>> {
    let mut x: Vec<f64> = Vec::new();
    let mut y: Vec<f64> = Vec::new();
    let mut reader = csv::Reader::from_reader(io::stdin());

    for result in reader.records() {
        let record = result?;
        x.push(record[0].parse()?);
        y.push(record[1].parse()?);
    }
    let root_drawing_area = BitMapBackend::new("plot.png", (900, 600)).into_drawing_area();
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
        .draw_series(
            x.into_iter()
                .zip(y)
                .map(|point| Cross::new(point, 3, Into::<ShapeStyle>::into(&BLUE).stroke_width(2))),
        )?
        .label("Cat")
        .legend(|(x, y)| Cross::new((x, y), 3, Into::<ShapeStyle>::into(&BLUE).stroke_width(2)));
    chart
        .configure_series_labels()
        .position(SeriesLabelPosition::LowerRight)
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;
    Ok(())
}
