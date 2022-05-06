use geo_clipper::Clipper;
// use std::convert::TryFrom;
// use geo_types::*;
// use geo::Polygon;
// use geo::algorithm::intersects::Intersects;
use geo::algorithm::area::*;

pub fn intersect(a: &Vec<geo::Polygon<f64>>, b: &Vec<geo::Polygon<f64>>) {

    let mut intersects = 0;
    let mut non_intersects = 0;

    let mut a_index = 0;
    while a_index < a.len() {
        let mut b_index = 0;
        while b_index < b.len() {
            let res = a[a_index].intersection(&b[b_index], 1000000.0);

            if res.signed_area() > 0. {
                intersects += 1;
            } else {
                non_intersects += 1;
            }

            b_index += 1;
        }
        a_index += 1;
    }
    println!("Intersection tests: {:?} Intersections: {:?} Non-intersections: {:?}", intersects + non_intersects, intersects, non_intersects);
}
