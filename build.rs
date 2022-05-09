use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use bincode;
use blender_mesh;
use landon;
use svg_load::svgload::load_svg;
use svg_load::ttfload::load_font;

const GLYPHS_REQUIRED: &'static str = "ABCDEFGHIJKLMNOPQRSTUVWXYSabcdefghijklmnopqrstuvwxyz0123456789,./'\\:;[]{}()!@#$%^&*~?-=_+ АБВГДЕЁЖЗИЙКЛМНОПРСТУФХЦЧШЩЪЫЬЭЮЯабвгдеёжзийклмнопрстуфхцчшщъыьэюя<>";

fn main() {
    let blender_files = vec![
        PathBuf::from(r"./terrain.blend"),
        PathBuf::from(r"./velodrome1.blend"),
    ];

    // // Only re-run this build script if we change our blender file
    for blender_file in blender_files.iter() {
        println!(
            "{}",
            format!("cargo:rerun-if-changed={}", blender_file.to_str().unwrap())
        );
    }

    // // Checks if `blender` is in your $PATH
    // let found_blender_executable = Command::new("command")
    //     .args(&["-v", "C:\\Program Files\\Blender Foundation\\Blender 3.1\\blender.exe"])
    //     .output()
    //     .unwrap()
    //     .stdout
    //     .len()
    //     > 0;
    //
    // if !found_blender_executable {
    //     return;
    // }

    let blender_stdout = landon::export_blender_data(&blender_files).unwrap();

    let meshes_by_file = blender_mesh::parse_meshes_from_blender_stdout(&blender_stdout);
    let flattened_meshes = blender_mesh::flatten_exported_meshes(&meshes_by_file).unwrap();
    let flattened_meshes = bincode::serialize(&flattened_meshes).unwrap();

    let mut f = File::create("./meshes.bytes").unwrap();
    f.write_all(&flattened_meshes[..]).unwrap();

    let armatures_by_file = blender_armature::parse_armatures_from_blender_stdout(&blender_stdout);

    let flattened_armatures =
        blender_armature::flatten_exported_armatures(&armatures_by_file).unwrap();

    let flattened_armatures = bincode::serialize(&flattened_armatures).unwrap();

    let mut f = File::create("./armatures.bytes").unwrap();
    f.write_all(&flattened_armatures[..]).unwrap();

    let mut images = HashMap::new();
    images.insert("test.svg", load_svg(r"./amc.svg"));
    images.insert("HR", load_svg(r"./heart1.svg"));
    let svg = bincode::serialize(&images).unwrap();
    let mut f = File::create("./svgs.bytes").unwrap();
    f.write_all(&svg[..]).unwrap();

    println!("Loading fonts...");

    let mut fonts = HashMap::new();
    fonts.insert("Roboto-Light", load_font(r"./Roboto-Light.ttf", GLYPHS_REQUIRED).unwrap());
    fonts.insert("SourceSansPro-Black", load_font(r"./SourceSansPro-Black.ttf", GLYPHS_REQUIRED).unwrap());

    let font = bincode::serialize(&fonts).unwrap();
    // let font_complessed = zstd::encode_all(&font[..],5).unwrap();
    let mut f = File::create("./fonts.bytes").unwrap();
    let mut b = brotli::CompressorWriter::new(f, 4096, 4, 20);
    b.write_all(&font[..]);
    b.flush();
    //f.write_all().unwrap();
}
