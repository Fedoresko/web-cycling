use std::borrow::BorrowMut;
use bincode;
//use blender_armature::BlenderArmature;
use blender_mesh::{BlenderMesh, CreateSingleIndexConfig, SingleIndexedVertexAttributes};
use std::collections::HashMap;
use std::io::Read;
use svg_load::font::Font;
use svg_load::path::RenderablePath;

#[derive(Default)]
pub struct Assets {
    meshes: HashMap<String, SingleIndexedVertexAttributes>,
    images: HashMap<String, Vec<RenderablePath>>,
    fonts: HashMap<String, Font>,
//    armatures: HashMap<String, BlenderArmature>,
}

impl Assets {
    pub fn new() -> Assets {
        let meshes = Assets::download_meshes();
        let images = Assets::download_images();
        let fonts = Assets::download_fonts();
//        let armatures = Assets::download_armatures();

        Assets { meshes, images, fonts }
    }

    fn download_images() -> HashMap<String, Vec<RenderablePath>> {
        let images = include_bytes!("../../../svgs.bytes");
        let images: HashMap<String, Vec<RenderablePath>> = bincode::deserialize(images).unwrap();

        for (font_name, font) in &images {
            web_sys::console::log_1(&format!("Loaded image... {}", font_name).into());


            for path in font {
                if path.gradient_stops > 0 {
                    web_sys::console::log_1(&format!("path gradient stops {} start: {}, {}; end {}, {}", path.gradient_stops, path.gradient_start.unwrap().0,
                                                     path.gradient_start.unwrap().1, path.gradient_end.unwrap().0, path.gradient_end.unwrap().1).into());
                }
                web_sys::console::log_1(&format!("ver {} ind {}", path.vertices.vertices.len(), path.vertices.indices.len()).into());

                let mut s = String::from("Points: ");
                for v in &path.vertices.vertices {
                    s += &format!("{:?}, ", v.position);
                }
                web_sys::console::log_1(&s.into());

                let mut s = String::from("Indxs: ");
                let mut max = 0;
                for v in &path.vertices.indices {
                    if *v > max {max = *v};
                    s += &format!("{:?}, ", v);
                }


            }
        };

        images
    }

    fn download_fonts() -> HashMap<String, Font> {
        let fonts_compressed = include_bytes!("../../../fonts.bytes");
        let mut de = brotli::Decompressor::new(&fonts_compressed[..], 4096);
        let mut fonts = Vec::new();
        de.borrow_mut().read_to_end(&mut fonts).expect("Decompressed fonts");
        let fontsm : HashMap<String, Font> = bincode::deserialize(&fonts.as_slice()).unwrap();
        fontsm
    }

    // In a real application you would download via XHR or fetch request, but here we just
    // included_bytes! for simplicity
    fn download_meshes() -> HashMap<String, SingleIndexedVertexAttributes> {
        let meshes = include_bytes!("../../../meshes.bytes");
        let mut meshes: HashMap<String, BlenderMesh> = bincode::deserialize(meshes).unwrap();
        let mut res: HashMap<String, SingleIndexedVertexAttributes> = HashMap::new();

        for (mesh_name, mesh) in meshes.iter_mut() {
            web_sys::console::log_1(&mesh_name.to_string().into());

            let arm_name = mesh.armature_name();
            if arm_name.is_none() {
                mesh.y_up();
            }

            let attrs = mesh.combine_vertex_indices(&CreateSingleIndexConfig::default());
            res.insert(mesh_name.clone(), attrs);
        }

        res
    }

    // In a real application you would download via XHR or fetch request, but here we just
    // included_bytes! for simplicity
    // fn download_armatures() -> HashMap<String, BlenderArmature> {
    //     let armatures = include_bytes!("../../../armatures.bytes");
    //     let mut armatures: HashMap<String, BlenderArmature> =
    //         bincode::deserialize(armatures).unwrap();
    //
    //     for (armature_name, armature) in armatures.iter_mut() {
    //         web_sys::console::log_1(&armature_name.to_string().into());
    //
    //         armature.apply_inverse_bind_poses();
    //         armature.transpose_actions();
    //         armature.matrices_to_dual_quats();
    //     }
    //
    //     armatures
    // }

    pub fn get_mesh(&self, mesh_name: &str) -> Option<&SingleIndexedVertexAttributes> {
        self.meshes.get(mesh_name)
    }

    pub fn get_image(&self, image_name: &str) -> Option<&Vec<RenderablePath>> {
        self.images.get(image_name)
    }

    pub fn get_font(&self, font_name: &str) -> Option<&Font> {
        self.fonts.get(font_name)
    }

    // pub fn get_armature(&self, armature_name: &str) -> Option<&BlenderArmature> {
    //     self.armatures.get(armature_name)
    // }
}
