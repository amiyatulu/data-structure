use plotters::prelude::*;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // Create a drawing area for the plot
    let root = SVGBackend::new("big_o_constant_factor.svg", (1280, 960)).into_drawing_area();
    root.fill(&WHITE)?;

    // Define the chart
    let mut chart = ChartBuilder::on(&root)
        .caption("Algorithm Complexity: O(5n) vs O(n)", ("sans-serif", 50))
        .margin(10)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0..30, 0..150)?;

    chart.configure_mesh().draw()?;

    // Algorithm A data (5 operations per element)
    let algorithm_a_data: Vec<(i32, i32)> = (0..=30).map(|n| (n, 5 * n)).collect();
    chart.draw_series(LineSeries::new(
        algorithm_a_data,
        &BLUE,
    ))?.label("Algorithm A (O(5n))").legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));

    // Algorithm B data (1 operation per element)
    let algorithm_b_data: Vec<(i32, i32)> = (0..=30).map(|n| (n, n)).collect();
    chart.draw_series(LineSeries::new(
        algorithm_b_data,
        &RED,
    ))?.label("Algorithm B (O(n))").legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    // Configure the legend
    chart.configure_series_labels().background_style(&WHITE.mix(0.8)).border_style(&BLACK).draw()?;

    // Save the drawing
    root.present()?;

    Ok(())
}