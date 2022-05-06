use std::time::{Instant};
use geojson::*;

mod data;
mod intersection;

#[tokio::main]
async fn main() {

    let now = Instant::now();

    // retrieve data as FeatureCollections
    let plots = data::get_plots().await;
    let zones = data::get_zones().await;

    println!("data retrieved after: {}", now.elapsed().as_millis());

    // convert plots to vectors of Polygons
    let mut plot_polygons = vec![];
    for plot_feature in &plots.features {
        if let Some(plot_geometry) = plot_feature.clone().geometry {
            match plot_geometry.value {
                Value::Polygon(_) => {
                    let plot_polygon: geo::Polygon<f64> = plot_geometry.value.try_into().expect("Unable to convert");
                    plot_polygons.push(plot_polygon);
                }
                _ => {}
            }
        }
    }

    println!("converted plots after: {}", now.elapsed().as_millis());

    // convert zones to vectors of Polygons
    let mut zone_polygons = vec![];
    for zone_feature in &zones.features {
        if let Some(zone_geometry) = zone_feature.clone().geometry {
            match zone_geometry.value {
                Value::Polygon(_) => {
                    let zone_polygon: geo::Polygon<f64> = zone_geometry.value.try_into().expect("Unable to convert");
                    zone_polygons.push(zone_polygon);
                }
                _ => {}
            }
        }
    }
    
    println!("zones converted after: {}", now.elapsed().as_millis());
    println!("nr plots {:?} nr zones {:?}", plot_polygons.len(), zone_polygons.len());

    intersection::geo_clipper::intersect(&plot_polygons, &zone_polygons);
    // intersection::geo::intersect(&plot_polygons, &zone_polygons);

    println!("Intersection test done after: {}", now.elapsed().as_millis());
}
