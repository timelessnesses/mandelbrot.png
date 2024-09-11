use ndarray::{linspace, Array, Array2};
use num_bigint::{BigInt, ToBigInt};
use num_complex::Complex;
use plotters::prelude::*;
use plotters::style::{BLACK, WHITE};

struct Z {
    z: BigInt,
    c: BigInt,
    limit: Option<u128>,
    count: u128,
}

impl Iterator for Z {
    type Item = BigInt;

    fn next(&mut self) -> Option<Self::Item> {
        let r = self.z.clone();
        if let Some(limit) = self.limit {
            if self.count > limit {
                return None;
            }
        }

        self.z = self.z.pow(2) + self.c.clone();

        self.count += 1;

        Some(r)
    }
}

#[derive(Default)]
struct ZBuilder {
    z: BigInt,
    candidate: BigInt,
    limit: Option<u128>,
}

impl ZBuilder {
    fn new() -> Self {
        ZBuilder::default()
    }

    fn z(self, z: i128) -> Self {
        ZBuilder {
            z: z.to_bigint().unwrap(),
            candidate: self.candidate,
            limit: self.limit,
        }
    }

    fn c(self, c: i128) -> Self {
        ZBuilder {
            z: self.z,
            candidate: c.to_bigint().unwrap(),
            limit: self.limit,
        }
    }

    fn limit(self, limit: u128) -> Self {
        ZBuilder {
            z: self.z,
            candidate: self.candidate,
            limit: Some(limit),
        }
    }

    fn build(self) -> Z {
        Z {
            z: self.z,
            c: self.candidate,
            limit: self.limit,
            count: 0,
        }
    }
}

fn mandelbrot(c: i128) -> ZBuilder {
    ZBuilder::new().z(0).c(c)
}

fn julia(c: i128, p: i128) -> ZBuilder {
    ZBuilder::new().z(c).c(p)
}

fn complex_matrix(
    x_min: f64,
    x_max: f64,
    y_min: f64,
    y_max: f64,
    pixel_density: i128,
) -> Array2<Complex<f64>> {
    let re = linspace(x_min, x_max, ((x_max - x_min) * pixel_density as f64) as usize)
        .collect::<Vec<f64>>();
    let im = linspace(y_min, y_max, ((y_max - y_min) * pixel_density as f64) as usize)
        .collect::<Vec<f64>>();

    let re_array = Array::from_vec(re);
    let im_array = Array::from_vec(im);

    let mut complex_array = Array2::from_elem((im_array.len(), re_array.len()), Complex::new(0.0, 0.0));

    for (i, im_val) in im_array.iter().enumerate() {
        for (j, re_val) in re_array.iter().enumerate() {
            complex_array[[i, j]] = Complex::new(*re_val, *im_val);
        }
    }

    complex_array
}

fn is_stable(c: &Complex<f64>, num_iterations: usize) -> bool {
    let mut z = Complex::new(0.0, 0.0);
    for _ in 0..num_iterations {
        z = z * z + c;
    }
    z.norm() <= 2.0
}

fn get_members(c: &Array2<Complex<f64>>, num_iterations: usize) -> Vec<(f64, f64)> {
    let mut members = Vec::new();

    for &value in c.iter() {
        if is_stable(&value, num_iterations) {
            members.push((value.re, value.im));
        }
    }

    members
}

fn plot_mandelbrot(members: Vec<(f64, f64)>) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("mandelbrot.png", (20000, 20000)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("Mandelbrot Set", ("sans-serif", 30).into_font())
        .margin(5)
        .x_label_area_size(20)
        .y_label_area_size(20)
        .build_cartesian_2d(-2.0..0.5, -1.5..1.5)?;

    chart.configure_mesh().disable_mesh().draw()?;

    chart.draw_series(
        members
            .into_iter()
            .map(|(re, im)| Circle::new((re, im), 1, &BLACK)),
    )?;

    root.present()?;
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let c = complex_matrix(-2.0, 0.5, -1.5, 1.5, 8000);
    let members = get_members(&c, 20);
    
    plot_mandelbrot(members)?;

    println!("Plot saved to mandelbrot.png");

    Ok(())
}
