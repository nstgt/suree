use regex::Regex;

pub fn parse(input: &str) -> Option<Vec<(String, String)>> {
    /* Regexp for commands
       This regexp is used to match command and description pairs like following format:

       $ cargo --help
       ...(snip)

       Commands:
         build, b    Compile the current package
         check, c    Analyze the current package and report errors, but don't build object files
         clean       Remove the target directory
         doc, d      Build this package's and its dependencies' documentation
         new         Create a new cargo package
         init        Create a new cargo package in an existing directory
         add         Add dependencies to a manifest file
         remove      Remove dependencies from a manifest file
         run, r      Run a binary or example of the local package
         ...(snip)

    */
    let re_command: Regex =
        Regex::new(r"^\s{2,}([,\w-]+\s?[\w-]+)\s{2,}([\S]+(?:\s{1}[\S]+)+)$").unwrap();

    let mut commands = Vec::new();

    for line in input.lines() {
        let trimmed_line = line.trim_start();

        // ignore flags
        if trimmed_line.starts_with('-') {
            continue;
        }

        // simple commands pattern
        if let Some(caps) = re_command.captures(line) {
            let command = caps.get(1).map_or("", |m| m.as_str()).to_string();
            let description = caps.get(2).map_or("", |m| m.as_str()).to_string();
            commands.push((command, description));
            continue;
        }
    }

    Some(commands)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;
    use std::fs;

    #[derive(Debug, Deserialize)]
    struct Command {
        command: String,
        description: String,
    }

    #[derive(Debug, Deserialize)]
    struct TestCase {
        command_output: String,
        parse_result: Vec<Command>,
    }

    fn list_case_file_paths() -> Result<Vec<String>, std::io::Error> {
        let mut yaml_files = Vec::new();
        for entry in std::fs::read_dir(concat!(env!("CARGO_MANIFEST_DIR"), "/resource/test"))? {
            let entry = entry?;
            let path = entry.path();
            if let Some(path_str) = path.to_str() {
                if path.is_file() && (path_str.ends_with(".yaml") || path_str.ends_with(".yml")) {
                    yaml_files.push(path_str.to_string());
                }
            }
        }

        Ok(yaml_files)
    }

    fn load_test_case_yaml_file(file_path: String) -> (String, Vec<(String, String)>) {
        let yaml_content = fs::read_to_string(file_path);

        let test_case: TestCase = serde_yaml::from_str(&yaml_content.unwrap()).unwrap();
        let expected = test_case
            .parse_result
            .into_iter()
            .map(|x| (x.command, x.description))
            .collect();

        (test_case.command_output, expected)
    }

    #[test]
    fn parse_test() {
        let files = list_case_file_paths().unwrap();
        for file in files {
            let (output, expected) = load_test_case_yaml_file(file);

            let parsed = parse(&output).unwrap();
            assert_eq!(parsed, expected)
        }
    }
}
