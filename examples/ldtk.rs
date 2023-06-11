use ldtk_rust::{Project, EntityInstance};

// Loads an LDtk Project file along with any external level files
// that it references.
fn main() {
    let file_path = "assets/levels/TheVeiledPath.ldtk".to_string();
    let project: Project = Project::new(file_path);
    let layers = project.levels[0].layer_instances.as_ref().unwrap();
    let ids_dimension_1: &Vec<i64> = &layers[0].int_grid_csv;
    let ids_dimension_2: &Vec<i64> = &layers[1].int_grid_csv;
    dbg!(ids_dimension_1);
    dbg!(ids_dimension_2);
}