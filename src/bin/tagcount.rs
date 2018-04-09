extern crate april;
extern crate image;

use april::*;

use std::env::args;


fn main() {
    println!("STARTING");
    let mut detector = Detector::create();
    detector.add_family(TagFamilyType::Tag16h5);
    detector.add_family(TagFamilyType::Tag25h7);
    detector.add_family(TagFamilyType::Tag25h9);
    detector.add_family(TagFamilyType::Tag36h10);
    detector.add_family(TagFamilyType::Tag36h11);
    println!("READY");

    let filename = args().nth(2).unwrap();
    println!("OPENING {}", filename);
    let img = image::open(filename).unwrap();
    println!("LOADED");
    let mut img: Image = image::imageops::grayscale(&img).into();
    println!("CONVERTED");
    //println!("{:?}", img);
    let detections = detector.detect(&mut img);
    println!("DETECTED");
    println!("FOUND {} TAGS", detections.len());
    for i in 0..detections.len() {
        let detect = detections.get(i);
        println!("{}\t{}\t{}\t{:?}", detect.id(), detect.goodness(), detect.margin(), detect.p());
    }
}
