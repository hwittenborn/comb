use crate::message;
use colored::Colorize;
use handlebars::Handlebars;
use std::{collections::HashMap, env, fs};

pub fn render(app: &clap::ArgMatches) {
    let file_arg: &String = app.get_one("file").unwrap();

    let file_content = match fs::read_to_string(file_arg) {
        Ok(file) => file,
        Err(err) => {
            message::error(&format!(
                "Failed to read '{}'. [{}]\n",
                file_arg.green().bold(),
                err.to_string().bold()
            ));
            quit::with_code(exitcode::IOERR);
        }
    };

    // Get the default data exposed to Handlebars templates.
    let mut data: HashMap<String, HashMap<String, String>> = HashMap::new();
    let mut env_data: HashMap<String, String> = HashMap::new();

    for (key, value) in env::vars() {
        env_data.insert(key, value);
    }
    data.insert("env".to_string(), env_data);

    // Render the template.
    let handlebars = Handlebars::new();

    let rendered_string = match handlebars.render_template(&file_content, &data) {
        Ok(rendered_string) => rendered_string,
        Err(err) => {
            message::error(&format!("{}", err));
            quit::with_code(exitcode::USAGE);
        }
    };

    println!("{}", rendered_string);
}
