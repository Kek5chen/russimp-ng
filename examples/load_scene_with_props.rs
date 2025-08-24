use russimp_ng::node::Node;
use russimp_ng::property::Property;
use russimp_ng::scene::PostProcess;
use russimp_ng::sys::AI_CONFIG_IMPORT_FBX_PRESERVE_PIVOTS;
use russimp_ng::{property::PropertyStore, scene::Scene};

fn traverse_nodes(node: &Node, indent: String) {
    println!("{}{}", indent, node.name);
    for child in node.children.borrow().iter() {
        traverse_nodes(&*child, format!("  {}", indent));
    }
}

fn main() {
    // NOTE: You can construct this from a HashMap
    // or any iterator if you want.
    //
    // The cast here is only necessary because
    // the array only contains one entry and rust
    // tries to make an iterator returning a reference
    // to a sized array instead of a slice.
    let props: PropertyStore = [(
        AI_CONFIG_IMPORT_FBX_PRESERVE_PIVOTS as &[u8],
        Property::Integer(0),
    )]
    .into_iter()
    .into();

    let scene = Scene::from_file_with_props(
        "models/FBX/y_bot_run.fbx",
        vec![
            PostProcess::Triangulate,
            PostProcess::GenerateSmoothNormals,
            PostProcess::FlipUVs,
            PostProcess::FlipWindingOrder,
            PostProcess::JoinIdenticalVertices,
            PostProcess::OptimizeGraph,
        ],
        &props,
    )
    .unwrap();

    if let Some(root) = &scene.root {
        traverse_nodes(&*root, String::from(""));
    }
}
