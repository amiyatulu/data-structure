use plotters::prelude::*;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // Create a drawing area for the plot
    let root = SVGBackend::new("big_o_constant_factor.svg", (1280, 960)).into_drawing_area();
    root.fill(&WHITE)?;
    // Define the chart
    let mut chart = ChartBuilder::on(&root)
        .caption("Difference Between O(10n) and O(n)", ("sans-serif", 50))
        .margin(30)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0..100000, 0..1000000)?;

    chart.configure_mesh().draw()?;

    // Data for O(5n)
    let data_5n: Vec<(i32, i32)> = (0..=100000).map(|n| (n, 10 * n)).collect();
    chart
        .draw_series(LineSeries::new(data_5n, &BLUE))?
        .label("O(10n)")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));

    // Data for O(n)
    let data_n: Vec<(i32, i32)> = (0..=100000).map(|n| (n, n)).collect();
    chart
        .draw_series(LineSeries::new(data_n, &RED))?
        .label("O(n)")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    // Data for the difference between O(5n) and O(n)
    let difference_data: Vec<(i32, i32)> = (0..=100000).map(|n| (n, 10 * n - n)).collect();
    chart
        .draw_series(LineSeries::new(difference_data, &GREEN))?
        .label("Difference (O(10n) - O(n))")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &GREEN));

    // Configure the legend
    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    // Save the drawing
    root.present()?;
    Ok(())
}
