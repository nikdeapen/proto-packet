use crate::args::{folder_arg, target_lang};
use clerr::Report;
use file_storage::FolderPath;
use proto_packet_compile::{Compiler, Language};

pub fn compile(lang: String, source: String, target: String) -> Result<(), Report> {
    let lang: Language = target_lang(lang.as_str())?;
    let source: FolderPath = folder_arg(source)?;
    let target: FolderPath = folder_arg(target)?;

    println!("Compiling ({}): {} -> {}", lang, source, target);

    Compiler::from(lang)
        .compile(&source, &target)
        .map_err(|e| e.into())
}
