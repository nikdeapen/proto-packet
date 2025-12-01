use crate::args::file_arg;
use crate::io::file_content;
use clerr::Report;
use file_storage::FilePath;
use lex::{Config, Context, Token};

pub fn parse(file: String) -> Result<(), Report> {
    let file: FilePath = file_arg(file)?;

    println!("Parsing: {}", file);

    let content: String = file_content(&file)?;
    let token: Token = Token::from(content.as_str());
    let config: Config = proto_packet_parse::config();
    let context: Context = Context::new(token, &config);
    match proto_packet_parse::parse_schema_file(context) {
        Ok(tree) => {
            println!("{:#?}", tree);
            Ok(())
        }
        Err(e) => {
            let token: Token = e.token();
            Err(e.to_error().to_report(file.as_str(), context, token))
        }
    }
}
