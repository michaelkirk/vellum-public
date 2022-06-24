use geo_clipper::Clipper;
use geo::algorithm::Area;
use geo::geometry::Polygon;

pub fn intersect(a: &Vec<Polygon>, b: &Vec<Polygon>) {

    let mut intersects = 0;
    let mut non_intersects = 0;

    let mut a_index = 0;
    while a_index < a.len() {
        let mut b_index = 0;
        while b_index < b.len() {
            let res = a[a_index].intersection(&b[b_index], 1000000.0);

            if res.unsigned_area() > 0. {
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
