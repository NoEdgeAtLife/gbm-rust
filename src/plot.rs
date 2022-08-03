
use plotters::prelude::*;
use rand::Rng;

/// prepare_vec returns a 2d vector suitable for plotting and also min, max values of input vector
pub fn prepare_vec(vals: Vec<f64>) -> (Vec<(f64, f64)>, f64, f64) {
    let mut out = vec![(0.0, 0.0); vals.len()];
    let mut min = vals[0];
    let mut max = vals[0];

    for i in 0..vals.len() {
        out[i] = (i as f64, vals[i]);
        if vals[i] > max {
            max = vals[i]
        } else if vals[i] < min {
            min = vals[i]
        }
    }
    return (out, min, max);
}

/// Plots the given values in a single plot to filename
/// returns an Error if there has been an error
/// Used for graphing the timeseries
pub fn plot_values(
    val: Vec<Vec<f64>>,
    filename: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new(filename, (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut vec3d = vec![];
    let mut min3d = vec![];
    let mut max3d = vec![];
    for i in 0..val.len(){
    let vals = val[i].clone();
    let (vec2d, min, max) = prepare_vec(vals);
    vec3d.push(vec2d.clone());
    min3d.push(min);
    max3d.push(max);
    }
    let min = min3d.iter().min_by(|a, b| a.total_cmp(b)).unwrap();
    let max = max3d.iter().max_by(|a, b| a.total_cmp(b)).unwrap();

    let mut chart = ChartBuilder::on(&root)
            .caption(filename, ("sans-serif", 30).into_font())
            .margin(10)
            .x_label_area_size(50)
            .y_label_area_size(50)
            .build_ranged(0f64..vec3d[0].len() as f64, *min..*max)?;

    //chart all the vec3d
    for i in 0..vec3d.len(){
        let mut rng1 = rand::thread_rng();
        let mut rng2 = rand::thread_rng();
        let mut rng3 = rand::thread_rng();
        chart.draw_series(LineSeries::new(vec3d[i].clone(),&RGBColor(rng1.gen_range(0..255),rng2.gen_range(0..255),rng3.gen_range(0..255))))?;
            //.label(filename.to_owned()+"_"+&i.to_string());
    }

    chart.configure_mesh().draw()?;

    //chart
    //.configure_series_labels()
    //.background_style(&WHITE.mix(0.8))
    //.border_style(&BLACK)
    //.draw()?;
    
    Ok(())
}
