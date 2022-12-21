use js_sys::Date;
use crate::fields::FieldSelector;
use crate::log;

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

pub struct AnimationSequence {
    animations: Vec<Box<dyn Animator>>,
    repeat: bool,
    current_animation: usize,
    current_started: f64,
}

pub trait Animator {
    fn animate(&mut self) -> Vec<FieldSelector>;
    fn get_target(&self) -> usize;
    fn is_finished(&self) -> bool;
    fn reset(&mut self);
}

static FAR_FUTURE: f64 = 1e20;

#[allow(dead_code)]
impl Animation {
    pub fn pause(target_id: usize, duration: f64) -> Animation {
        Animation {
            target_id,
            duration,
            from : FieldSelector::None,
            to : FieldSelector::None,
            repeat: false,
            started: Date::now(),
            act: Box::new(|pg, from, to, | { FieldSelector::None }),
        }
    }

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

impl AnimationSequence {
    pub fn new(animations: Vec<Box<dyn Animator>>, repeat: bool) -> AnimationSequence {
        AnimationSequence {
            animations,
            repeat,
            current_animation: 0,
            current_started: Date::now(),
        }
    }
}

impl Animator for AnimationSequence {
    fn animate(&mut self) -> Vec<FieldSelector> {
        let len = self.animations.len();
        let mut cur = self.animations.get_mut(self.current_animation).unwrap();
        let res = cur.animate();
        if cur.is_finished() {
            if self.current_animation < len -1 {
                log!("Animation change");
                self.current_animation += 1;
                cur = self.animations.get_mut(self.current_animation).unwrap();
                cur.reset();
            } else if self.repeat {
                log!("Animation reset");
                self.current_animation = 0;
                cur = self.animations.get_mut(self.current_animation).unwrap();
                cur.reset();
            }
        }
        res
    }

    fn get_target(&self) -> usize {
        self.animations.get(0).unwrap().get_target()
    }

    fn is_finished(&self) -> bool {
        !self.repeat && self.current_animation == self.animations.len() - 1 &&
            self.animations.get(self.current_animation).unwrap().is_finished()
    }

    fn reset(&mut self) {
        self.current_animation = 0;
        self.animations.get_mut(self.current_animation).unwrap().reset();
    }
}

impl Animator for Animation {
    fn animate(&mut self) -> Vec<FieldSelector> {
        let time = Date::now();
        let progress = (time - self.started) / self.duration;
        if progress > 0.0 && progress <= 1.0 {
            vec![(self.act)(progress, self.from, self.to)]
        } else if progress > 1.0 {
            if self.repeat {
                self.started = Date::now();
                vec![(self.act)(0.0, self.from, self.to)]
            } else {
                self.started = FAR_FUTURE;
                vec![(self.act)(1.0, self.from, self.to)]
            }
        } else {
            vec![(self.act)(0.0, self.from, self.to)]
        }
    }

    fn get_target(&self) -> usize {
        self.target_id
    }

    fn is_finished(&self) -> bool {
        return self.started == FAR_FUTURE
    }

    fn reset(&mut self) {
        self.started = Date::now();
    }
}

impl Animator for CompositeAnimation {
    fn animate(&mut self) -> Vec<FieldSelector> {
        let mut res = Vec::new();
        for anim in &mut self.animations {
            res.extend(anim.animate());
        }
        res
    }

    fn get_target(&self) -> usize {
        self.animations.get(0).unwrap().get_target()
    }

    fn is_finished(&self) -> bool {
        self.animations.iter().map(|a| a.is_finished()).reduce(|a,b| a && b).unwrap()
    }

    fn reset(&mut self) {
        for anim in &mut self.animations {
            anim.reset();
        }
    }
}

