use std::collections::HashMap;

use proto_packet_tree::{ModPathRef, Project, SchemaFile, TypeNameRef, WithTypeName};

use crate::{Error, SchemaLinker};

/// Responsible for linking projects.
#[derive(Debug, Default)]
pub struct ProjectLinker {
    _nothing: (),
}

impl ProjectLinker {
    //! Link

    /// Links the `project`.
    pub fn link(&self, project: &Project) -> Result<Project, Error> {
        let all_names: HashMap<ModPathRef, Vec<TypeNameRef>> = self.all_names(project);

        let mut result: Project = Project::default();
        for (mod_path, schema_file) in project.schema_files() {
            let linker: SchemaLinker = unsafe { SchemaLinker::new(mod_path.to_ref(), &all_names) };
            let schema_file: SchemaFile = linker.link(schema_file)?;
            unsafe { result.add_schema_file(mod_path.clone(), schema_file) };
        }
        Ok(result)
    }

    /// Gets all the declared names from the `project`.
    fn all_names<'b>(&self, project: &'b Project) -> HashMap<ModPathRef<'b>, Vec<TypeNameRef<'b>>> {
        let mut all_names: HashMap<ModPathRef<'b>, Vec<TypeNameRef<'b>>> = HashMap::default();
        for (mod_path, schema_file) in project.schema_files() {
            for type_dec in schema_file.type_decs() {
                all_names
                    .entry(mod_path.to_ref())
                    .or_insert(Vec::default())
                    .push(type_dec.type_name())
            }
        }
        all_names
    }
}
