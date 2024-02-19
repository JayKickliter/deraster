use gdal::Dataset;
use rasterh3::{AxisOrder, H3Converter, ResolutionSearchMode, Transform};
use std::path::PathBuf;

fn main() {
    let input: PathBuf = dbg!([env!("CARGO_MANIFEST_DIR"), "assets", "S20W180.hgt"]
        .iter()
        .collect());
    let dataset = Dataset::open(input).unwrap();
    let geo_transform = &dataset.geo_transform().unwrap();
    let transform = Transform::from_gdal(geo_transform);
    let band = dataset.rasterband(1).unwrap();
    let band_size @ (cols, rows) = band.size();
    assert_eq!(cols, 1201);
    assert_eq!(rows, 1201);
    let band_array = band
        .read_as_array::<i16>((0, 0), band_size, band_size, None)
        .unwrap();
    let view = band_array.view();
    let conv = H3Converter::new(&view, &Some(i16::MIN), &transform, AxisOrder::YX);
    let res = dbg!(conv
        .nearest_h3_resolution(ResolutionSearchMode::MinDiff)
        .unwrap());
    let cells: Vec<_> = conv
        .to_h3(res, true)
        .unwrap()
        .into_iter()
        .map(|(&val, cells)| (val, cells.into_compacted_iter().collect::<Vec<_>>()))
        .collect();
    println!("{}", cells.len());
}
