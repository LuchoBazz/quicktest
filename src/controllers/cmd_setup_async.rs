use exitfailure::ExitFailure;

use crate::{
    cli::model::setup_command::SetupCommand,
    config::load_config::{read_language_configuration, write_language_configuration},
    constants::LANGUAGE_CONFIG_FILE,
    error::handle_error::throw_setup_label_is_not_correct_msg,
    file_handler::async_file::remove_files_async,
    language::json::language_scheme::Languages,
    views::{setup::show_argument_was_updated_success, style::show_config_file_deleted_path},
};

pub struct SetupController {
    command: SetupCommand,
}

impl SetupController {
    pub fn new(command: SetupCommand) -> SetupController {
        SetupController { command }
    }

    pub async fn run(&mut self) -> Result<(), ExitFailure> {
        if !self.has_correct_structure() {
            let label = self.get_label();
            return throw_setup_label_is_not_correct_msg(label);
        }

        let langs = self.get_languages();

        if langs.is_err() {
            let label = self.get_label();
            return throw_setup_label_is_not_correct_msg(label);
        }

        let mut lg = langs.unwrap();

        let cmds = self.get_commands();
        let language_key = cmds[0];
        let label = cmds[1];
        let value = self.get_value();

        for lang in &mut lg.languages {
            if lang.id == language_key && lang.env.contains_key(label) {
                lang.env.insert(label.to_string(), value.to_string());
                write_language_configuration(&lg)?;

                show_argument_was_updated_success(language_key, label, value);

                return Ok(());
            }
        }
        Ok(())
    }

    fn get_languages(&self) -> serde_json::Result<Languages> {
        let text: &str = &read_language_configuration()[..];
        serde_json::from_str(text)
    }

    fn get_commands(&self) -> Vec<&str> {
        let cmds: Vec<&str> = self.command.label.split('.').collect();
        cmds
    }

    fn get_label(&self) -> &str {
        &self.command.label[..]
    }

    fn get_value(&self) -> &str {
        &self.command.value[..]
    }

    fn has_correct_structure(&self) -> bool {
        let cmds = self.get_commands();
        cmds.len() == 2
    }

    // fn is_language_by_label()

    pub async fn reset() -> Result<(), ExitFailure> {
        let config_file = &shellexpand::tilde(LANGUAGE_CONFIG_FILE).to_string()[..];
        if remove_files_async(vec![config_file]).await.is_ok() {
            show_config_file_deleted_path(config_file);
        }
        Ok(())
    }

    // Show Help setup after help: quicktest setup config --help
    // EXAMPLES:
    //     quicktest setup config --label="Language::Cpp.PROGRAM" --value="g++"
    //     quicktest setup config --label="Language::Cpp.STANDARD" --value="-std=c++17"
    //     quicktest setup config --label="Language::Python.PROGRAM" --value="python"
    pub fn show_help_setup() -> &'static str {
        let text = read_language_configuration();

        let mut langs: serde_json::Result<Languages> = serde_json::from_str(&text[..]);

        let mut labels = vec!["EXAMPLES:".to_string()];

        if let Ok(lg) = &mut langs {
            for idx in 0..lg.languages.len() {
                for (key, value) in &lg.languages[idx].env {
                    let label = format!(
                        "    quicktest setup config --label=\"{}.{}\" --value=\"{}\"",
                        lg.languages[idx].id, key, value
                    );
                    labels.push(label);
                }
            }
        }
        let labels: String = labels.join("\n");
        Box::leak(labels.into_boxed_str())
    }
}
