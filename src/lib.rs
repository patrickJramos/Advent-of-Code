use day01trebuchet::trebuchet_2;
mod day01trebuchet;
mod day02colored_cubes;

use day02colored_cubes::colored_cubes;

pub fn run(challenge: String, input: String) -> Result<String, String> {
    match challenge.as_str() {
        "trebuchet" => Ok(trebuchet_2(input).to_string()),
        "colored_cubes" => Ok(colored_cubes(input.as_str()).to_string()),

        _ => Err(format!("Unknown challenge: {}", challenge)),
    }
}
