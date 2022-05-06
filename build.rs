use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use bincode;
use blender_mesh;
use landon;
use svg_load::svgload::load_svg;

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

    let _svg = load_svg(r"./test.svg");
    let svg = bincode::serialize(&_svg).unwrap();
    let mut f = File::create("./svgs.bytes").unwrap();
    f.write_all(&svg[..]).unwrap();
}
