use js_sys::Date;
use std::ops::{Add, Mul, Sub};

#[allow(dead_code)]
pub struct Animation<'a, T> {
    target: &'a mut T,
    duration: f64,
    repeat: bool,
    act: Box<dyn Fn(f64) -> T + 'a>,
    started: f64,
}

#[allow(dead_code)]
impl<'a, T> Animation<'a, T>
where
    T: Sub<Output = T> + Add<Output = T> + Mul<f64, Output = T> + Copy + 'a,
{
    const FAR_FUTURE: f64 = 1e20;

    pub fn linear(target: &'a mut T, from: T, to: T, duration: f64) -> Animation<'a, T> {
        Animation {
            target,
            duration,
            repeat: false,
            started: Self::FAR_FUTURE,
            act: Box::new(move |pg| from + (to - from) * pg),
        }
    }

    pub fn fade_in_out(target: &'a mut T, from: T, to: T, duration: f64) -> Animation<'a, T> {
        Animation {
            target,
            duration,
            repeat: false,
            started: Self::FAR_FUTURE,
            act: Box::new(move |pg| from + (to - from) * (1.0 - (pg * 2.0 - 1.0).abs())),
        }
    }

    pub fn start(&mut self) {
        self.started = Date::now();
    }

    pub fn animate(&mut self) {
        let time = Date::now();
        let progress = (time - self.started) / self.duration;
        if progress > 0.0 && progress <= 1.0 {
            *(self.target) = (self.act)(progress);
        } else if progress > 1.0 {
            if self.repeat {
                self.start();
            } else {
                self.started = Self::FAR_FUTURE;
            }
        }
    }
}
