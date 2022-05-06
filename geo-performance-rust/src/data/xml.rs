// use quick_xml::Reader;
// use quick_xml::events::Event;
use geojson::*;
// use log::*;
// use crate::constants::*;
use xmltree::*;
// use std::fs::File;
use serde_json::*;

// use super::colors;

pub fn to_json(xml_str:&String, feature_type:&String) -> FeatureCollection {
    let mut collection = FeatureCollection {
        bbox:None,
        features: vec![],
        foreign_members: None,
    };
    let wfs_features_collection = Element::parse(xml_str.as_bytes()).unwrap();

    for member in wfs_features_collection.children {
        match member {
            XMLNode::Element(element) => {
                let xml_feature = element.get_child(feature_type.to_string()).unwrap();
                let feature = Feature {
                    bbox: None,
                    geometry: Some(to_geometry(xml_feature.get_child("geometrie").unwrap())),
                    id: None,
                    properties: Some(Map::new()),
                    foreign_members: None,
                };
                collection.features.push(feature);
            },
            _ => {}
        }
    }

    collection
}
pub fn to_geometry(geometry: &Element) -> Geometry {
    let polygon = geometry.get_child("Polygon").unwrap();
    let exterior = polygon.get_child("exterior").unwrap();
    let linring = exterior.get_child("LinearRing").unwrap();
    let poslist = linring.get_child("posList").unwrap();
    let text_coordinates = poslist.get_text().unwrap();
    // warn!("polygon {:?}", text_coordinates);

    let arr_coordinates:Vec<&str> = text_coordinates.split_whitespace().collect();

    let mut index = 0;
    let mut coordinates = vec![vec![]];
    while index < arr_coordinates.len() {
        coordinates[0].push(
            vec![
                arr_coordinates[index].parse::<f64>().unwrap_or(0.),
                arr_coordinates[index + 1].parse::<f64>().unwrap_or(0.),
            ]
        );
        index += 2;
    }
    Geometry::new(geojson::Value::Polygon(coordinates))
}
