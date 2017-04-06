
use super::raw_vec::RawVec;
use super::plan::Plan;
use super::enums::*;
use super::r2r::*;

use num_complex::Complex64 as c64;
use std::ops::Mul;

pub struct Pair<A, B> {
    pub field: RawVec<A>,
    pub coef: RawVec<B>,
    logical_size: usize,
    forward: Plan<A, B>,
    backward: Plan<B, A>,
}

impl<A, B> Pair<A, B> {
    pub fn logical_size(&self) -> usize { self.logical_size }

    pub fn forward(&mut self) {
        unsafe {
            self.forward.execute();
        }
    }

    pub fn backward(&mut self) {
        unsafe {
            self.backward.execute();
        }
    }

    pub fn normalize_field_by(&mut self, factor: f64)
        where A: Mul<f64, Output = A> + Copy
    {
        for val in self.field.iter_mut() {
            *val = *val * factor;
        }
    }

    pub fn normalize_coef_by(&mut self, factor: f64)
        where B: Mul<f64, Output = B> + Copy
    {
        for val in self.coef.iter_mut() {
            *val = *val * factor;
        }
    }
}

impl Pair<f32, f32> {
    pub fn r2r_1d(n: usize, kind: R2R_KIND, flag: FLAG) -> Self {
        let mut field = RawVec::<f32>::new(n);
        let mut coef = RawVec::<f32>::new(n);
        let forward = Plan::<f32, f32>::r2r_1d(n, &mut field, &mut coef, forward(kind), flag);
        let backward = Plan::<f32, f32>::r2r_1d(n, &mut coef, &mut field, backward(kind), flag);
        Pair {
            field: field,
            coef: coef,
            logical_size: logical_size(n, kind),
            forward: forward,
            backward: backward,
        }
    }
}

impl Pair<f64, f64> {
    pub fn r2r_1d(n: usize, kind: R2R_KIND, flag: FLAG) -> Self {
        let mut field = RawVec::<f64>::new(n);
        let mut coef = RawVec::<f64>::new(n);
        let forward = Plan::<f64, f64>::r2r_1d(n, &mut field, &mut coef, forward(kind), flag);
        let backward = Plan::<f64, f64>::r2r_1d(n, &mut coef, &mut field, backward(kind), flag);
        Pair {
            field: field,
            coef: coef,
            logical_size: logical_size(n, kind),
            forward: forward,
            backward: backward,
        }
    }
}

impl Pair<f64, c64> {
    pub fn r2c_1d(n: usize, flag: FLAG) -> Self {
        let mut field = RawVec::<f64>::new(n);
        let mut coef = RawVec::<c64>::new(n / 2 + 1);
        let forward = Plan::r2c_1d(n, &mut field, &mut coef, flag);
        let backward = Plan::c2r_1d(n, &mut coef, &mut field, flag);
        Pair {
            field: field,
            coef: coef,
            logical_size: n,
            forward: forward,
            backward: backward,
        }
    }
}

impl Pair<c64, c64> {
    pub fn c2c_1d(n: usize, sign: SIGN, flag: FLAG) -> Self {
        let mut field = RawVec::<c64>::new(n);
        let mut coef = RawVec::<c64>::new(n);
        let forward = Plan::c2c_1d(n, &mut field, &mut coef, sign, flag);
        let backward = Plan::c2c_1d(n, &mut coef, &mut field, -sign, flag);
        Pair {
            field: field,
            coef: coef,
            logical_size: n,
            forward: forward,
            backward: backward,
        }
    }
}
