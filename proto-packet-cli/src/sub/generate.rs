use clerr::Report;
use clerr::Severity::Info;
use colored::Colorize;
use file_storage::{FilePath, FolderPath};
use lex::{Config, ParseContext, Token};

use proto_packet_gen::{Generated, Generator};
use proto_packet_tree::{ModPath, Project};
use proto_packet_validate::validate_schema_file;

use crate::args::{create_generator, file_arg};
use crate::io::file_content;

pub fn generate(lang: String, file: String) -> Result<(), Report> {
    let generator: Box<dyn Generator> = create_generator(lang.as_str())?;
    let file: FilePath = file_arg(file)?;

    println!("Generating ({}): {}", lang, file);

    let content: String = file_content(&file)?;
    let token: Token = Token::from(content.as_str());
    let config: Config = proto_packet_parse::create_parse_config();
    let context: ParseContext = ParseContext::new(token, &config);
    match proto_packet_parse::parse_schema_file(context) {
        Ok(tree) => match validate_schema_file(&tree) {
            Ok(schema_file) => {
                let mut project: Project = Project::default();
                project.add_schema(unsafe { ModPath::new_unchecked("root") }, schema_file);

                let target: FolderPath = FolderPath::unix_root();
                let generated: Generated = generator.generate(&project, &target)?;

                for (path, source) in generated.sources() {
                    println!("{}:", path.as_str().color(Info.color()));
                    source.split("\n").for_each(|line| {
                        print!("\t");
                        println!("{}", line);
                    });
                }

                Ok(())
            }
            Err(e) => Err(e.report(file.as_str(), context)),
        },
        Err(e) => {
            let token: Token = e.token();
            Err(e.to_error().report(file.as_str(), context, token))
        }
    }
}
