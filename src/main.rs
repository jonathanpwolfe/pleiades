mod linear_regression;
mod datapoint;
use linear_regression::LinearRegression;
use datapoint::DataPoint;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Example dataset: (x, y) pairs for the scatter plot
    let data = vec![
        DataPoint::new( 1.0, 2.0 ),
        DataPoint::new( 2.0,4.0 ),
        DataPoint::new(  3.0,5.0 ),
        DataPoint::new(4.0, 4.0 ),
        DataPoint::new( 5.0, 5.0 ),
        DataPoint::new( 5.0, 4.0 ),
        DataPoint::new( 1.5, 2.5 ),
        DataPoint::new(3.2, 4.8 ),
        DataPoint::new(4.8, 5.3 ),
    ];
    let mut lr = LinearRegression::new(&data);
    lr.calculate();
    println!("{:?}",lr);
    lr.plot(
        "Scatter Plot with Linear Regression",
        "X Axis",
        "Y Axis",
        "scatter_plot.png",
    )?;

    Ok(())
}