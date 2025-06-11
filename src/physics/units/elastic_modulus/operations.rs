use crate::physics::units::elastic_modulus::{ElasticModulus, ElasticModulusUnit};
use std::ops::{Add, Div, Mul, Neg, Sub};

impl<U: ElasticModulusUnit> Add for ElasticModulus<U> {
    type Output = ElasticModulus<U>;
    fn add(self, other: ElasticModulus<U>) -> ElasticModulus<U> {
        ElasticModulus::new(self.value + other.value)
    }
}

impl<U: ElasticModulusUnit> Sub for ElasticModulus<U> {
    type Output = ElasticModulus<U>;
    fn sub(self, other: ElasticModulus<U>) -> ElasticModulus<U> {
        ElasticModulus::new(self.value - other.value)
    }
}

impl<U: ElasticModulusUnit> Mul<f64> for ElasticModulus<U> {
    type Output = ElasticModulus<U>;
    fn mul(self, scalar: f64) -> ElasticModulus<U> {
        ElasticModulus::new(self.value * scalar)
    }
}

impl<U: ElasticModulusUnit> Div<f64> for ElasticModulus<U> {
    type Output = ElasticModulus<U>;
    fn div(self, scalar: f64) -> ElasticModulus<U> {
        ElasticModulus::new(self.value / scalar)
    }
}

impl<U: ElasticModulusUnit> Neg for ElasticModulus<U> {
    type Output = ElasticModulus<U>;
    fn neg(self) -> ElasticModulus<U> {
        ElasticModulus::new(-self.value)
    }
}
