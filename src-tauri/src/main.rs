#![cfg_attr(
    all(not(debug_assertions), target_os = "linux"),
    windows_subsystem = "linux"
)]

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

enum CalcError {}

#[tauri::command]
fn calculate(numbers: &str) -> String {
    fn get_sum_of_string_numbers(string: &str) -> f32 {
        return string
            .split(",")
            .map(|x| x.trim())
            .map(|x| x.parse::<f32>().unwrap())
            .sum();
    }

    let perform = || -> Result<f32, CalcError> {
        let get_sum: f32 = get_sum_of_string_numbers(numbers);
        Ok(get_sum)
    };


    match perform() {
      Ok(val) => {
        return format!("Calculated value is: {}", val)
      },
      Err(_e) => {
        return format!("Error occured")
      }
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, calculate])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
