use std::collections::HashMap;

use code_gen::{CodeBuffer, Source, Statement};

use proto_packet_tree::SourceDec::MessageDec;
use proto_packet_tree::{SchemaFile, WithName};

use crate::rust::{GenMessage, Naming, Typing};
use crate::{GenError, Generator};

/// Responsible for generating the rust code for schema file.
#[derive(Clone, Debug, Default)]
pub struct RustGenerator {
    naming: Naming,
    typing: Typing,
}

impl Generator for RustGenerator {
    fn generate(&self, schema_file: &SchemaFile) -> Result<HashMap<String, String>, GenError> {
        let mut map: HashMap<String, String> = HashMap::default();
        for source_dec in schema_file.declarations() {
            match source_dec {
                MessageDec(message) => {
                    let file_name: String = self.naming.file_name(message.name())?;

                    let gen: GenMessage = GenMessage::new(&self.naming, &self.typing);
                    let source: Source = gen.gen(message)?;
                    let mut b: CodeBuffer = CodeBuffer::default();
                    source.write(&mut b, 0);
                    let source: String = b.export();

                    map.insert(file_name, source);
                }
            }
        }
        Ok(map)
    }
}
