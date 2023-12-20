use std::io::Write;

use rand::thread_rng;
use rand_distr::{Distribution, Normal, NormalError};

const CHART_WIDTH: usize = 75;

#[derive(Debug)]
struct DistriPlot {
    // middle and dev means a normal distribution of N(middle, dev)
    middle: f64,
    dev: f64,
    // left and right are default to middle - 3 * dev, middle + 3 * dev
    // TODO: using tuple (left, right)
    left: f64,
    right: f64,
    // segs and seg_len are caculated, determined by how precisely you want to draw
    segs: u64,
    seg_width: f64,
    bars: Vec<u64>,
    experi_times: u64,
}

impl DistriPlot {
    fn new(middle: f64, dev: f64, segs: u64) -> Self {
        let left = middle - 3.0 * dev;
        let right = middle + 3.0 * dev;
        let seg_width = (right - left) / segs as f64;
        let middle = left / 2.0 + (right - left) / 2.0;

        DistriPlot {
            left,
            right,
            middle,
            segs,
            dev,
            seg_width,
            bars: vec![0; segs as usize],
            experi_times: 1000,
        }
    }

    fn cal_bars(&mut self) -> Result<(), NormalError> {
        let mut rng = thread_rng();
        let normal = Normal::new(self.middle, self.dev)?;
        for _i in 0..self.experi_times {
            let v = normal.sample(&mut rng);
            if v < self.left || v > self.right {
                continue;
            }
            let seg_index = ((v - self.left) / self.seg_width) as usize;
            self.bars[seg_index] += 1;
        }

        Ok(())
    }

    fn draw(&self) {
        for (i, &b) in self.bars.iter().enumerate() {
            let percent = b as f64 / self.experi_times as f64 * 2.0;
            print!("{:.1}: ", self.left + i as f64 * self.seg_width);
            draw_bar(percent, CHART_WIDTH);
        }
    }
}

// println! with fill
// println!(":<fill><align><width>")
// 1$ means the first argument in println!
fn draw_bar(progress: f64, width: usize) {
    // Calculate the number of characters to fill based on the progress
    let num_filled = (progress * width as f64) as usize;

    // ANSI escape code for setting text color to green
    print!("\x1b[32m[");

    // Print the filled portion of the bar
    // for _ in 0..num_filled {
    //     print!("█");
    // }
    print!("{:█<1$}", "", num_filled);

    // ANSI escape code for resetting text color
    print!("\x1b[0m");

    // Print the empty portion of the bar
    for _ in num_filled..width {
        print!(" ");
    }
    // print!("{:x<1$}", "", width-num_filled);

    // Print the progress percentage
    println!("] {:.1}%", progress * 100.0);
    // Flush the output to make sure it's displayed immediately
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rust_floats() {
        let a = 4.0_f64;
        let b = 0.3_f64;

        assert_eq!((a / b) as i64, 13);
    }

    #[test]
    fn test_normal_distri() {
        use rand::thread_rng;
        use rand_distr::{Distribution, Normal};

        let mut rng = thread_rng();
        let normal = Normal::new(2.0, 3.0).expect("Error creating normal distribution");
        let v = normal.sample(&mut rng);

        println!("{} is from a N(2, 9) distribution", v);
    }

    #[test]
    fn test_plot() {
        let mut plt = DistriPlot::new(3.0, 1.0, 21);
        plt.cal_bars().expect("bars init error");
        println!("{:?}", plt.bars);
        plt.draw();
    }

    #[test]
    fn test_draw_bar() {
        draw_bar(0.5, 50);
    }

}
