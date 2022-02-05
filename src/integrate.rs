use crate::config::Config;
use crate::functions::shuberts;
use std::sync::mpsc;
use std::thread;
use std::time::Instant;
use libm::{fma};

struct IntegrateRegion {
    x0: f64,
    x1: f64,
    y0: f64,
    y1: f64,
    xsteps: f64,
    ysteps: f64,
}

#[derive(Debug)]
pub struct IntegrateResult {
    pub integral: f64,
    pub time_ms: u128,
    pub abs_err: f64,
    pub rel_err: f64,
}

fn abs(first: f64) -> f64 {
    if first > 0f64 {
        first
    } else {
        -first
    }
}

fn one_thread_integrate(reg: IntegrateRegion, tx: mpsc::Sender<f64>) -> () {
    let dx = (reg.x1 - reg.x0) / reg.xsteps;
    let dy = (reg.y1 - reg.y0) / reg.ysteps;
    let mut result = 0f64;
    let (mut i, mut j) = (0f64, 0f64);
    while i < reg.xsteps {
        while j < reg.ysteps {
            result += shuberts(((i + 0.5) * dx) + reg.x0, ((j+0.5) * dy) + reg.y0);
            j += 1f64;
        }
        j = 0f64;
        i += 1f64;
    }
    tx.send(result * dx * dy).unwrap();
}

pub fn parallel_integrate(cnf: &Config) -> (f64, u128) {
    let start_time = Instant::now();

    let deltay = (cnf.y1 - cnf.y0) / (cnf.thread_num as f64);
    let mut y = cnf.y0;

    let mut handles = vec![];
    let (tx, rx) = mpsc::channel();
    for _ in 0..cnf.thread_num - 1 {
        let region = IntegrateRegion {
            x0: cnf.x0,
            x1: cnf.x1,
            y0: y,
            y1: y + deltay,
            xsteps: cnf.xsteps as f64,
            ysteps: (cnf.ysteps as f64) / (cnf.thread_num as f64),
        };
        let new_tx = tx.clone();
        let handle = thread::spawn(move || one_thread_integrate(region, new_tx));
        handles.push(handle);
        y += deltay;
    }

    let region = IntegrateRegion {
        x0: cnf.x0,
        x1: cnf.x1,
        y0: y,
        y1: cnf.y1,
        xsteps: cnf.xsteps as f64,
        ysteps: (cnf.ysteps as f64) / (cnf.thread_num as f64),
    };
    let handle = thread::spawn(move || one_thread_integrate(region, tx.clone()));
    handles.push(handle);

    let mut res = 0f64;
    for received in rx {
        res += received;
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let elapsed = start_time.elapsed().as_millis();
    return (res, elapsed);
}

pub fn parallel_integrate_err(config: &mut Config) -> IntegrateResult {
    let (mut cur_integ, mut prev_integ) = (0f64, 0f64);
    let (mut abs_err, mut rel_err) = (0f64, 0f64);
    let mut time_ms = 0;

    for _ in 0..config.max_iters {
        let int_res = parallel_integrate(&config);
        cur_integ = int_res.0;
        time_ms += int_res.1;

        abs_err = abs(cur_integ - prev_integ);
        rel_err = abs(abs_err / cur_integ);

        if abs_err < config.abs_error && rel_err < config.rel_error {
            break;
        }
        config.xsteps *= 2;
        config.ysteps *= 2;
        prev_integ = cur_integ;
        println!("time: {}, abs_error: {}, rel_error: {}", int_res.1, abs_err, rel_err);
    }

    IntegrateResult {
        integral: cur_integ,
        abs_err,
        rel_err,
        time_ms,
    }
}
