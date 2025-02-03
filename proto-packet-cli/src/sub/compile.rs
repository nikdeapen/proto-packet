use clerr::Report;
use file_storage::FolderPath;

use proto_packet_compile::Compiler;
use proto_packet_gen::Generator;

use crate::args::{create_generator, folder_arg};

pub fn compile(lang: String, source: String, target: String) -> Result<(), Report> {
    let generator: Box<dyn Generator> = create_generator(lang.as_str())?;
    let source: FolderPath = folder_arg(source)?;
    let target: FolderPath = folder_arg(target)?;

    println!("Compiling ({}): {} into {}", lang, source, target);

    Compiler::from(generator)
        .compile(&source, &target)
        .map_err(|e| e.to_report())
}
