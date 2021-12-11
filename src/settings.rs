use serde_derive::Deserialize;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
pub struct SettingData {
    opecity: f32,
    input_files: Vec<String>,
    output_suffix: String,
}

impl SettingData {
    pub fn load(file_path: &str) -> SettingData {
        let file_data = fs::read_to_string(file_path).unwrap();
        toml::from_str(&file_data).unwrap()
    }

    pub fn get_alpha_value(&self, alpha: u8) -> u8 {
        let base: f32 = 100.0;
        let result: f32 = (self.opecity / base) * (alpha as f32);
        result as u8
    }

    pub fn get_files(&self) -> Vec<(String, String)> {
        let mut output_paths: Vec<(String, String)> = vec![];
        for input_file in &self.input_files {
            let mut input_path_buffer = PathBuf::from(input_file);
            let file_name = input_path_buffer
                .file_stem()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string();
            let extension = input_path_buffer
                .extension()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string();

            let output_file_name = file_name + "_" + &self.output_suffix;
            input_path_buffer.set_file_name(output_file_name);
            input_path_buffer.set_extension(extension);

            output_paths.push((
                format!("{}", input_file),
                format!("{}", input_path_buffer.to_str().unwrap()),
            ));
            //             let input_path = Path::new(&input_file);
            //             println!("{:?}", input_path.file_name().unwrap());
            //             println!("{:?}", input_path.file_stem().unwrap());
            //             println!("{:?}", input_path.extension().unwrap());
            //             let file_name = input_path.file_name();
        }

        output_paths
    }
}

#[cfg(test)]

mod tests {

    #[test]
    fn deserialize() {
        let toml_data = r#"
opecity = 100
input_files = ["./sample.png"]
output_suffix = "state"
"#;
        let setting_data: super::SettingData = toml::from_str(&toml_data).unwrap();
        //assert_ !(setting_data.opecity, 100.0);
        assert_eq!(setting_data.input_files[0], "./sample.png");
        assert_eq!(setting_data.output_suffix, "state");
    }

    #[test]
    fn file_load() {
        super::SettingData::load("./settings.toml");
    }

    #[test]
    fn output_path() {
        let toml_data = r#"
opecity = 100
input_files = ["./sample.png"]
output_suffix = "state"
"#;
        let settting_data: super::SettingData = toml::from_str(&toml_data).unwrap();

        let output_files = settting_data.get_files();
        assert_eq!(output_files[0].0, "./sample.png");
        assert_eq!(output_files[0].1, "./sample_state.png");
    }
}
