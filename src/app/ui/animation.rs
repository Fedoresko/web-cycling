use js_sys::Date;
use crate::fields::FieldSelector;
use crate::element::Element;

#[allow(dead_code)]
pub struct Animation {
    target_id: usize,
    duration: f64,
    from: FieldSelector,
    to: FieldSelector,
    repeat: bool,
    act: Box<dyn Fn(f64, FieldSelector, FieldSelector) -> FieldSelector>,
    started: f64,
}

pub struct CompositeAnimation {
    pub animations: Vec<Box<dyn Animator>>,
}

pub trait Animator {
    fn animate(&mut self, elem: &mut Element);
    fn get_target(&self) -> usize;
    fn is_finished(&self) -> bool;
}

static FAR_FUTURE: f64 = 1e20;

#[allow(dead_code)]
impl Animation {
    pub fn linear(target_id: usize, from: FieldSelector, to: FieldSelector, duration: f64) -> Animation {
        Animation {
            target_id,
            duration,
            from,
            to,
            repeat: false,
            started: Date::now(),
            act: Box::new(|pg, from, to, | {from + (to - from) * pg}),
        }
    }

    pub fn fade_in_out(target_id: usize, from: FieldSelector, to: FieldSelector, duration: f64) -> Animation {
        Animation {
            target_id,
            from,
            to,
            duration,
            repeat: false,
            started: Date::now(),
            act: Box::new(|pg, from, to| from + (to - from) * (1.0 - (pg * 2.0 - 1.0).abs())),
        }
    }
}

impl Animator for Animation {
    fn get_target(&self) -> usize {
        self.target_id
    }

    fn animate(&mut self, elem: &mut Element) {
        // if elem.get_id() == self.target_id {
            let time = Date::now();
            let progress = (time - self.started) / self.duration;
            if progress > 0.0 && progress <= 1.0 {
                elem.set((self.act)(progress, self.from, self.to) );
            } else if progress > 1.0 {
                if self.repeat {
                    self.started = Date::now();
                } else {
                    self.started = FAR_FUTURE;
                }
            }
        // } else {
        //     console::log_1(&format!("Wrong coincedence {} and {}",self.target_id, elem.get_id()).into());
        // }
    }

    fn is_finished(&self) -> bool {
        return self.started == FAR_FUTURE
    }
}

impl Animator for CompositeAnimation {
    fn animate(&mut self, elem: &mut Element) {
        for anim in &mut self.animations {
            anim.animate(elem);
        }
    }

    fn get_target(&self) -> usize {
        self.animations.get(0).unwrap().get_target()
    }

    fn is_finished(&self) -> bool {
        self.animations.iter().map(|a| a.is_finished()).reduce(|a,b| a && b).unwrap()
    }
}

