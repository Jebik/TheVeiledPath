use ldtk_rust::{Project, EntityInstance};

// Loads an LDtk Project file along with any external level files
// that it references.
fn main() {
    let file_path = "assets/levels/TheVeiledPath.ldtk".to_string();
    let project: Project = Project::new(file_path);
    let layers = project.levels[0].layer_instances.as_ref().unwrap();
    let ids_dimension_1 = &layers[0].int_grid_csv;
    let ids_dimension_2 = &layers[1].int_grid_csv;

    let width = layers[0].c_wid as usize;
    println!("dimension 1:");
    for gv in ids_dimension_1.iter().enumerate() {
        if *gv.1 != 0 {
            println!("{:?}: {}", [gv.0 / width, gv.0 % width], gv.1);
        }
    }
    println!("dimension 2:");
    for  gv in ids_dimension_2.iter().enumerate() {
        if *gv.1 != 0 {
            println!("{:?}: {}", [gv.0 / width, gv.0 % width], gv.1);
        }
    }
}