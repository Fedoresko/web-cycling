use bincode;
//use blender_armature::BlenderArmature;
use blender_mesh::{BlenderMesh, CreateSingleIndexConfig, SingleIndexedVertexAttributes};
use std::collections::HashMap;

#[derive(Default)]
pub struct Assets {
    meshes: HashMap<String, SingleIndexedVertexAttributes>,
//    armatures: HashMap<String, BlenderArmature>,
}

impl Assets {
    pub fn new() -> Assets {
        let meshes = Assets::download_meshes();
//        let armatures = Assets::download_armatures();

        Assets { meshes }
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

    // pub fn get_armature(&self, armature_name: &str) -> Option<&BlenderArmature> {
    //     self.armatures.get(armature_name)
    // }
}
