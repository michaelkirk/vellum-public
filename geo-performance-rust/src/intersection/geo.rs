use geojson::*;
use geo::algorithm::intersects::Intersects;

pub fn intersect(a: &Vec<geo::Polygon<f64>>, b: &Vec<geo::Polygon<f64>>) {
    
    let mut intersects = 0;
    let mut non_intersects = 0;

    let mut a_index = 0;
    while a_index < a.len() {
        let mut b_index = 0;
        while b_index < b.len() {
            if a[a_index].intersects(&b[b_index]){
                intersects += 1;
            } else {
                non_intersects += 1;
            }

            b_index += 1;
        }
        a_index += 1;
    }
    println!("Intersections: {:?} {:?}", intersects, non_intersects);

}