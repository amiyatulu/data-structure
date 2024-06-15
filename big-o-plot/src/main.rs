use plotters::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root = SVGBackend::new("plot.svg", (1280, 960)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("Function Plots", ("sans-serif", 50).into_font())
        .margin(30)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0.0..100.0, 0.0..10000.0)?;

    chart.configure_mesh().draw()?;

    // chart
    //     .draw_series(LineSeries::new(
    //         (1..=1000).map(|n| (n as f64, (n as f64).ln())),
    //         &RED,
    //     ))?
    //     .label("log(n)")
    //     .legend(|(x, y)| PathElement::new([(x, y), (x + 10, y)], &RED));

    // chart
    //     .draw_series(LineSeries::new(
    //         (1..=1000).map(|n| (n as f64, n as f64)),
    //         &GREEN,
    //     ))?
    //     .label("n")
    //     .legend(|(x, y)| PathElement::new([(x, y), (x + 20, y)], &GREEN));

    // chart
    //     .draw_series(LineSeries::new(
    //         (1..=1000).map(|n| (n as f64, (n as f64) * (n as f64).ln())),
    //         &BLUE,
    //     ))?
    //     .label("n log(n)")
    //     .legend(|(x, y)| PathElement::new([(x, y), (x + 20, y)], &BLUE));

    chart
        .draw_series(LineSeries::new(
            (1..=100).map(|n| (n as f64, (n as f64).powi(2))),
            &CYAN,
        ))?
        .label("n^2")
        .legend(|(x, y)| PathElement::new([(x, y), (x + 20, y)], &CYAN));

    // chart
    //     .draw_series(LineSeries::new(
    //         (1..=20).map(|n| (n as f64, (n as f64).powi(3))),
    //         &MAGENTA,
    //     ))?
    //     .label("n^3")
    //     .legend(|(x, y)| PathElement::new([(x, y), (x + 20, y)], &MAGENTA));

    // chart
    //     .draw_series(LineSeries::new(
    //         (1..=20).map(|n| (n as f64, (2.0f64).powf(n as f64))),
    //         &BLACK,
    //     ))?
    //     .label("2^n")
    //     .legend(|(x, y)| PathElement::new([(x, y), (x + 20, y)], &BLACK));

    chart.configure_series_labels().border_style(&BLACK).draw()?;

    Ok(())
}
