use std::rc::Rc;
use js_sys::Date;

pub trait TimeSeries<T> {
    fn fetch_data(self: &Rc<Self>, start_time: usize, end_time: usize, step: f32) -> Box<dyn Iterator<Item=T>>;
}

pub struct HrmData {
    pub data: Vec<(usize, f32)>,

    //iters : Vec<Box<TimeSeriesIterator<f32>>>,
}

struct HrmDataIter {
    data: Rc<HrmData>,
    start_time: usize,
    end_time: usize,
    step: f32,
    last_t: f32,
    pos: usize,
}

impl TimeSeries<f32> for HrmData {
    fn fetch_data(self: &Rc<Self>, start_time: usize, end_time: usize, step: f32) -> Box<dyn Iterator<Item=f32>> {
        let mut pos = 0;
        let mut last_t = self.data.get(pos).unwrap().0 as f32;
        while last_t < start_time as f32 && pos < self.data.len() {
            pos += 1;
            last_t = self.data.get(pos).unwrap().0 as f32;
        }
        let iter = HrmDataIter {
            data: self.clone(),
            start_time,
            end_time,
            step,
            last_t,
            pos,
        };
        //self.iters.push(Box::new(iter));
        Box::new(iter)
    }
}

impl HrmData {
    pub fn add_hr(&mut self, val: f32) {
        self.data.push((Date::now() as usize, val));
    }
}

impl Iterator for HrmDataIter {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.data.data.len() >= self.pos {
            return None;
        }
        while self.data.data.len() < self.pos && (self.data.data.get(self.pos).unwrap().0 as f32) < self.last_t + self.step {
            self.pos += 1;
        }
        let t: &(usize, f32) = self.data.data.get(self.pos).unwrap();
        if t.0 > self.end_time {
            None
        } else {
            self.last_t += self.step;
            Some(t.1)
        }
    }
}