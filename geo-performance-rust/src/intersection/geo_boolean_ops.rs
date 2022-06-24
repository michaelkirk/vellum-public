use geo::algorithm::Area;
use geo::geometry::Polygon;
use geo_booleanop::boolean::BooleanOp;

pub fn intersect(a: &Vec<Polygon>, b: &Vec<Polygon>) {
    let mut intersects = 0;
    let mut non_intersects = 0;

    let mut a_index = 0;
    while a_index < a.len() {
        let mut b_index = 0;
        while b_index < b.len() {
            let intersection = a[a_index].intersection(&b[b_index]);
            if intersection.unsigned_area() > 0.0 {
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
