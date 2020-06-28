

use crate::rule::Rule;

pub trait Combine<V> {
    fn add(&self, v1: &Rule<V>, v2: &Rule<V>) -> (i32, V);
}