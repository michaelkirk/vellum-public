use geo_clipper::Clipper;
use std::convert::TryFrom;
use geo_types::*;
use geo::Polygon;
use geo::algorithm::intersects::Intersects;

pub fn intersect(a: &Vec<geo::Polygon<f64>>, b: &Vec<geo::Polygon<f64>>) {
    let mut a_index = 0;
    while a_index < a.len() {
        let mut b_index = 0;
        while b_index < b.len() {
            let res = a[a_index].intersection(&b[b_index], 1.0);

            b_index += 1;
        }
        a_index += 1;
    }
}