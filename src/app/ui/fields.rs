use std::fmt::{Display, Formatter};
use std::ops::{Add, Index, Mul, Sub};
use std::slice::Iter;

/// Field Selector serves for modifying mutable properties of elements
/// currently only first three points of gradient supported
#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
pub enum FieldSelector {
    X(i32),
    Y(i32),
    Width(u32),
    Height(u32),
    BGColor(Vec4),
    GradientPos0(f32),
    GradientColors0(Vec4),
    GradientPos1(f32),
    GradientColors1(Vec4),
    GradientPos2(f32),
    GradientColors2(Vec4),
    GradientPos3(f32),
    GradientColors3(Vec4),
    GradientStart((f32,f32)),
    GradientEnd((f32,f32)),
    LabelText(SizedStr),
    LabelColor(Vec4),
}

pub type SizedStr = [char; 256];

pub trait Sizing<T>: Sized {
    fn sizify(_:T) -> Self;
}

impl Sizing<&str> for SizedStr {
    fn sizify(v: &str) -> Self {
        let mut t : [char; 256] = [' '; 256];
        for (k, ch) in v.chars().take(256).enumerate() {
            t[k] = ch;
        }
        t
    }
}

impl Display for FieldSelector {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Add for FieldSelector {
    type Output = FieldSelector;

    fn add(self, rhs: Self) -> Self::Output {
        match self {
            FieldSelector::X(v) => { if let FieldSelector::X(other) = rhs { FieldSelector::X(v + other) } else { panic!("wrong other type") } }
            FieldSelector::Y(v) => { if let FieldSelector::Y(other) = rhs { FieldSelector::Y(v + other) } else { panic!("wrong other type") } }
            FieldSelector::Width(v) => { if let FieldSelector::Width(other) = rhs { FieldSelector::Width((v + other) as u32) } else { panic!("wrong other type") } }
            FieldSelector::Height(v) => { if let FieldSelector::Height(other) = rhs { FieldSelector::Height((v + other) as u32) } else { panic!("wrong other type") } }
            FieldSelector::BGColor(v) => { if let FieldSelector::BGColor(other) = rhs { FieldSelector::BGColor(v + other) } else { panic!("wrong other type") } }
            FieldSelector::GradientPos0(v) => { if let FieldSelector::GradientPos0(other) = rhs { FieldSelector::GradientPos0(v + other) } else { panic!("wrong other type") } }
            FieldSelector::GradientColors0(v) => { if let FieldSelector::GradientColors0(other) = rhs { FieldSelector::GradientColors0(v + other) } else { panic!("wrong other type") } }
            FieldSelector::GradientPos1(v) => { if let FieldSelector::GradientPos1(other) = rhs { FieldSelector::GradientPos1(v + other) } else { panic!("wrong other type") } }
            FieldSelector::GradientColors1(v) => { if let FieldSelector::GradientColors1(other) = rhs { FieldSelector::GradientColors1(v + other) } else { panic!("wrong other type") } }
            FieldSelector::GradientPos2(v) => { if let FieldSelector::GradientPos2(other) = rhs { FieldSelector::GradientPos2(v + other) } else { panic!("wrong other type") } }
            FieldSelector::GradientColors2(v) => { if let FieldSelector::GradientColors2(other) = rhs { FieldSelector::GradientColors2(v + other) } else { panic!("wrong other type") } }
            FieldSelector::GradientPos3(v) => { if let FieldSelector::GradientPos3(other) = rhs { FieldSelector::GradientPos3(v + other) } else { panic!("wrong other type") } }
            FieldSelector::GradientColors3(v) => { if let FieldSelector::GradientColors3(other) = rhs { FieldSelector::GradientColors3(v + other) } else { panic!("wrong other type") } }
            FieldSelector::GradientStart(v) => { if let FieldSelector::GradientStart(other) = rhs { FieldSelector::GradientStart( (v.0 + other.0, v.1 + other.1) ) } else { panic!("wrong other type") } }
            FieldSelector::GradientEnd(v) => { if let FieldSelector::GradientEnd(other) = rhs { FieldSelector::GradientEnd((v.0 + other.0, v.1 + other.1)) } else { panic!("wrong other type") } }
            FieldSelector::LabelColor(v) => { if let FieldSelector::LabelColor(other) = rhs { FieldSelector::LabelColor(v + other) } else { panic!("wrong other type") } }
            FieldSelector::LabelText(v) => { panic!("cant add label text") }
        }
    }
}

impl  Sub for FieldSelector {
    type Output = FieldSelector;

    fn sub(self, rhs: Self) -> Self::Output {
        match self {
            FieldSelector::X(v) => { if let FieldSelector::X(other) = rhs { FieldSelector::X(v - other) } else { panic!("wrong other type") } }
            FieldSelector::Y(v) => { if let FieldSelector::Y(other) = rhs { FieldSelector::Y(v - other) } else { panic!("wrong other type") } }
            FieldSelector::Width(v) => { if let FieldSelector::Width(other) = rhs { FieldSelector::Width((v - other) as u32) } else { panic!("wrong other type") } }
            FieldSelector::Height(v) => { if let FieldSelector::Height(other) = rhs { FieldSelector::Height((v - other) as u32) } else { panic!("wrong other type") } }
            FieldSelector::BGColor(v) => { if let FieldSelector::BGColor(other) = rhs { FieldSelector::BGColor(v - other) } else { panic!("wrong other type") } }
            FieldSelector::GradientPos0(v) => { if let FieldSelector::GradientPos0(other) = rhs { FieldSelector::GradientPos0(v - other) } else { panic!("wrong other type") } }
            FieldSelector::GradientColors0(v) => { if let FieldSelector::GradientColors0(other) = rhs { FieldSelector::GradientColors0(v + other) } else { panic!("wrong other type") } }
            FieldSelector::GradientPos1(v) => { if let FieldSelector::GradientPos1(other) = rhs { FieldSelector::GradientPos1(v - other) } else { panic!("wrong other type") } }
            FieldSelector::GradientColors1(v) => { if let FieldSelector::GradientColors1(other) = rhs { FieldSelector::GradientColors1(v + other) } else { panic!("wrong other type") } }
            FieldSelector::GradientPos2(v) => { if let FieldSelector::GradientPos2(other) = rhs { FieldSelector::GradientPos2(v - other) } else { panic!("wrong other type") } }
            FieldSelector::GradientColors2(v) => { if let FieldSelector::GradientColors2(other) = rhs { FieldSelector::GradientColors2(v - other) } else { panic!("wrong other type") } }
            FieldSelector::GradientPos3(v) => { if let FieldSelector::GradientPos3(other) = rhs { FieldSelector::GradientPos3(v - other) } else { panic!("wrong other type") } }
            FieldSelector::GradientColors3(v) => { if let FieldSelector::GradientColors3(other) = rhs { FieldSelector::GradientColors3(v - other) } else { panic!("wrong other type") } }
            FieldSelector::GradientStart(v) => { if let FieldSelector::GradientStart(other) = rhs { FieldSelector::GradientStart((v.0 - other.0, v.1 - other.1)) } else { panic!("wrong other type") } }
            FieldSelector::GradientEnd(v) => { if let FieldSelector::GradientEnd(other) = rhs { FieldSelector::GradientEnd((v.0 - other.0, v.1 - other.1)) } else { panic!("wrong other type") } }
            FieldSelector::LabelColor(v) => { if let FieldSelector::LabelColor(other) = rhs { FieldSelector::LabelColor(v - other) } else { panic!("wrong other type") } }
            FieldSelector::LabelText(v) => { panic!("cant subtract label text") }
        }
    }
}

impl  Mul<f64> for FieldSelector {
    type Output = FieldSelector;

    fn mul(self, rhs: f64) -> Self::Output {
        match self {
            FieldSelector::X(v) => { FieldSelector::X( (v as f64 * rhs) as i32) }
            FieldSelector::Y(v) => { FieldSelector::Y( (v as f64 * rhs) as i32) }
            FieldSelector::Width(v) => { FieldSelector::Width( (v as f64 * rhs) as u32) }
            FieldSelector::Height(v) => { FieldSelector::Height( (v as f64 * rhs) as u32) }
            FieldSelector::BGColor(v) => { FieldSelector::BGColor(v * rhs) }
            FieldSelector::GradientPos0(v) => { FieldSelector::GradientPos0(v * rhs as f32) }
            FieldSelector::GradientColors0(v) => { FieldSelector::GradientColors0(v * rhs) }
            FieldSelector::GradientPos1(v) =>  { FieldSelector::GradientPos1(v * rhs as f32) }
            FieldSelector::GradientColors1(v) => { FieldSelector::GradientColors1(v * rhs) }
            FieldSelector::GradientPos2(v) =>  { FieldSelector::GradientPos2(v * rhs as f32) }
            FieldSelector::GradientColors2(v) => { FieldSelector::GradientColors2(v * rhs) }
            FieldSelector::GradientPos3(v) =>  { FieldSelector::GradientPos3(v * rhs as f32) }
            FieldSelector::GradientColors3(v) => { FieldSelector::GradientColors3(v * rhs) }
            FieldSelector::GradientStart(v) => { FieldSelector::GradientStart( (v.0 * rhs as f32, v.1 * rhs as f32) ) }
            FieldSelector::GradientEnd(v) => { FieldSelector::GradientEnd( (v.0 * rhs as f32, v.1 * rhs as f32) ) }
            FieldSelector::LabelColor(v) => { FieldSelector::LabelColor(v * rhs) }
            FieldSelector::LabelText(v) => { panic!("cant multiply label text") }
        }
    }
}

#[derive(Debug)]
pub struct Vec4 {
    val : [f32; 4],
}

impl Vec4 {
    pub fn as_slice(&self) -> &[f32] {
        &self.val[..]
    }
}


impl Default for Vec4 {
    fn default() -> Self {
        Vec4 {
            val: [0.0,0.0,0.0,0.0]
        }
    }
}

impl From<[f32; 4]> for Vec4 {
    fn from(s: [f32; 4]) -> Self {
        Vec4{val: s}
    }
}

impl<'a> IntoIterator for &'a Vec4 {
    type Item = &'a f32;
    type IntoIter = Iter<'a, f32>;

    fn into_iter(self) -> Self::IntoIter {
        let val = &self.val;
        val.into_iter()
    }
}

impl Clone for Vec4 {
    fn clone(&self) -> Self {
        Vec4::from(self.val.clone())
    }
}

impl Copy for Vec4 {}

impl Index<usize> for Vec4 {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.val[index]
    }
}

impl Into<[f32; 4]> for Vec4 {
    fn into(self) -> [f32; 4] {
        self.val
    }
}

impl Sub for Vec4 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec4::from([self[0]-rhs[0], self[1]-rhs[1], self[2]-rhs[2], self[3]-rhs[3]])
    }
}

impl Add for Vec4 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vec4::from([self[0]+rhs[0], self[1]+rhs[1], self[2]+rhs[2], self[3]+rhs[3]])
    }
}

impl Mul<f64> for Vec4 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec4::from([self[0]*rhs as f32, self[1]*rhs as f32, self[2]*rhs as f32, self[3]*rhs as f32])
    }
}
