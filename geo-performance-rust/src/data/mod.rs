
use geojson::*;

mod xml;

pub async fn get_plots() -> FeatureCollection {
    let url = "https://geodata.nationaalgeoregister.nl/kadastralekaart/wfs/v4_0?request=GetFeature&service=WFS&srsName=EPSG:4326&typeName=kadastralekaartv4:perceel&version=2.0.0&outputFormat=json&bbox=165593,480993,166125,481552";
    let json = reqwest::get(url).await.unwrap().json().await.unwrap();
    FeatureCollection::from_json_value(json).unwrap()
}
pub async fn get_zones() -> FeatureCollection {
    let url = "https://afnemers.ruimtelijkeplannen.nl/afnemers/services?request=GetFeature&service=WFS&srsName=EPSG:4326&typeName=Enkelbestemming&version=2.0.0&bbox=165618,480983,166149,481542";
    let xml = reqwest::get(url).await.unwrap().text().await.unwrap();
    xml::to_json(&xml, &"Enkelbestemming".to_string())
}