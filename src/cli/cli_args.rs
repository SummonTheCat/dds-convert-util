// D:/Work/Projects/Modding/Skyrim/dds-edit/src/cli/cli_args.rs

use crate::structs::struct_config::Config;
use std::env;

pub mod cli_args {
    use super::*;

    /// Parses command-line arguments and returns a `Config` struct.
    ///
    /// # Returns
    ///
    /// * `Ok(Config)` if parsing succeeds.
    /// * `Err(String)` with an error message if parsing fails.
    pub fn parse_arguments() -> Result<Config, String> {
        let args: Vec<String> = env::args().collect();
        let mut config = Config::default();

        let mut i = 1; // Start from 1 to skip the program name
        while i < args.len() {
            let arg = &args[i];

            if arg.starts_with("--") {
                // Handle long options
                if let Some((key, value)) = parse_long_option(arg) {
                    match key.as_str() {
                        "pathout" => {
                            config.path_out = value.ok_or("Missing value for --pathout")?;
                        }
                        "maxdepth" => {
                            let val = value.ok_or("Missing value for --maxdepth")?;
                            config.max_depth = val.parse::<usize>().map_err(|_| "Invalid value for --maxdepth")?;
                        }
                        "target-widths" => {
                            let val = value.ok_or("Missing value for --target-widths")?;
                            config.target_sizes = parse_target_widths(&val)?;
                        }
                        "threads" => {
                            let val = value.ok_or("Missing value for --threads")?;
                            config.threads = val.parse::<usize>().map_err(|_| "Invalid value for --threads")?;
                        }
                        "useconfig" => {
                            config.config_file = Some(value.ok_or("Missing value for --useconfig")?);
                        }
                        _ => return Err(format!("Unknown option '{}'", arg)),
                    }
                } else {
                    // If there's no '=', the value is in the next argument
                    let key = arg.trim_start_matches("--");
                    i += 1;
                    if i >= args.len() {
                        return Err(format!("Missing value for '--{}'", key));
                    }
                    let value = &args[i];
                    match key {
                        "pathout" => {
                            config.path_out = value.clone();
                        }
                        "maxdepth" => {
                            config.max_depth = value.parse::<usize>().map_err(|_| "Invalid value for --maxdepth")?;
                        }
                        "target-widths" => {
                            config.target_sizes = parse_target_widths(value)?;
                        }
                        "threads" => {
                            config.threads = value.parse::<usize>().map_err(|_| "Invalid value for --threads")?;
                        }
                        "useconfig" => {
                            config.config_file = Some(value.clone());
                        }
                        _ => return Err(format!("Unknown option '--{}'", key)),
                    }
                }
            } else if arg.starts_with("-") {
                // Handle short options like -o, -d, -t, -n, -c
                let chars: Vec<char> = arg.chars().collect();
                let mut j = 1;
                while j < chars.len() {
                    match chars[j] {
                        'o' => {
                            i += 1;
                            if i >= args.len() {
                                return Err("Missing value for -o".to_string());
                            }
                            config.path_out = args[i].clone();
                        }
                        'd' => {
                            i += 1;
                            if i >= args.len() {
                                return Err("Missing value for -d".to_string());
                            }
                            config.max_depth = args[i].parse::<usize>().map_err(|_| "Invalid value for -d")?;
                        }
                        't' => {
                            i += 1;
                            if i >= args.len() {
                                return Err("Missing value for -t".to_string());
                            }
                            config.target_sizes = parse_target_widths(&args[i])?;
                        }
                        'n' => {
                            i += 1;
                            if i >= args.len() {
                                return Err("Missing value for -n".to_string());
                            }
                            config.threads = args[i].parse::<usize>().map_err(|_| "Invalid value for -n")?;
                        }
                        'c' => {
                            i += 1;
                            if i >= args.len() {
                                return Err("Missing value for -c".to_string());
                            }
                            config.config_file = Some(args[i].clone());
                        }
                        _ => return Err(format!("Unknown option '-{}'", chars[j])),
                    }
                    j += 1;
                }
            } else {
                // Positional arguments
                if config.path_in.is_empty() {
                    config.path_in = arg.clone();
                } else {
                    return Err(format!("Unexpected positional argument '{}'", arg));
                }
            }
            i += 1;
        }

        // Ensure that the input path is provided unless a config file is set
        if config.path_in.is_empty() && config.config_file.is_none() {
            return Err("Input path is required.".to_string());
        }

        Ok(config)
    }

    /// Parses a long option in the format '--key=value'.
    ///
    /// # Arguments
    ///
    /// * `arg` - The argument string to parse.
    ///
    /// # Returns
    ///
    /// * `Some((key, Some(value)))` if a key and value are found.
    /// * `Some((key, None))` if only a key is found.
    /// * `None` if the argument is not a long option.
    fn parse_long_option(arg: &str) -> Option<(String, Option<String>)> {
        if arg.starts_with("--") {
            let mut parts = arg[2..].splitn(2, '=');
            let key = parts.next()?.to_string();
            let value = parts.next().map(|s| s.to_string());
            Some((key, value))
        } else {
            None
        }
    }

    /// Parses a comma-separated list of target widths into a vector of u32.
    ///
    /// # Arguments
    ///
    /// * `widths_str` - A string slice containing comma-separated widths.
    ///
    /// # Returns
    ///
    /// * `Ok(Vec<u32>)` if parsing succeeds.
    /// * `Err(String)` with an error message if parsing fails.
    fn parse_target_widths(widths_str: &str) -> Result<Vec<u32>, String> {
        widths_str
            .split(',')
            .map(|s| s.trim().parse::<u32>().map_err(|_| format!("Invalid target width '{}'", s.trim())))
            .collect()
    }
}
