use clerr::Report;
use clerr::Severity::Info;
use colored::Colorize;
use file_storage::{FilePath, FolderPath};
use lex::{Config, ParseContext, Token};

use proto_packet_compile::Language;
use proto_packet_gen::config::GenConfig;
use proto_packet_gen::rust::GenRust;
use proto_packet_gen::{CodeGenerator, GeneratedCode};
use proto_packet_tree::{ModPath, Project};
use proto_packet_validate::validate_schema_file;

use crate::args::{file_arg, target_lang};
use crate::io::file_content;

pub fn generate(lang: String, file: String) -> Result<(), Report> {
    let lang: Language = target_lang(lang.as_str())?;
    let file: FilePath = file_arg(file)?;

    println!("Generating ({:?}): {}", lang, file);

    let content: String = file_content(&file)?;
    let token: Token = Token::from(content.as_str());
    let config: Config = proto_packet_parse::config();
    let context: ParseContext = ParseContext::new(token, &config);
    let generator: Box<dyn CodeGenerator> = match lang {
        Language::Rust => Box::new(GenRust::new(GenConfig::default())),
    };
    match proto_packet_parse::parse_schema_file(context) {
        Ok(tree) => match validate_schema_file(&tree) {
            Ok(schema_file) => {
                let mut project: Project = Project::default();
                unsafe { project.add_schema_file(ModPath::new_unchecked("root"), schema_file) };

                let target: FolderPath = FolderPath::unix_root();
                let generated: GeneratedCode = generator.generate(&project, &target)?;

                for (path, source) in generated.sources() {
                    println!("{}:", path.as_str().color(Info.color()));
                    source.split("\n").for_each(|line| {
                        print!("\t");
                        println!("{}", line);
                    });
                }

                Ok(())
            }
            Err(e) => Err(e.to_report(file.as_str(), context)),
        },
        Err(e) => {
            let token: Token = e.token();
            Err(e.to_error().to_report(file.as_str(), context, token))
        }
    }
}
