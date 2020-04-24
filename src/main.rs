
use std::error::Error;
use std::io::{self, Write};
use std::path::Path;
use std::process::{Command, Stdio};
use clap::{load_yaml, App};
use clap_to_gui::run_gui;

fn read_tests<S: AsRef<Path>>(path: S) -> Result<Vec<(String, String)>, Box<dyn Error>> {
    let mut tests: Vec<(String, String)> = Vec::new();
    let mut rdr = csv::Reader::from_path(path)?;
    for result in rdr.records() {
        let record = result?;
        let input = record.get(0).unwrap().replace("\\n", "\n");
        let output = record.get(1).unwrap().replace("\\n", "\n");
        println!("{:?}", record);
        tests.push((input, output));
    }
    Ok(tests)
}

fn run_test(baseline: &Path, input: &str, expected_output: &str) -> bool {
    let mut child = Command::new(baseline)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to spawn process.");
    {
        let stdin = child.stdin.as_mut().expect("Failed to open stdin");
        stdin
            .write_all(input.as_bytes())
            .expect("Failed to write to stdin");
    }
    let output = child.wait_with_output().expect("Failed to read stdout");
    //println!("STDIN: ");
    //io::stdout().write_all(&output.stdin).unwrap();
    println!("Expected STDOUT: ");
    io::stdout().write_all(&expected_output.as_bytes()).unwrap();
    println!("STDOUT: ");
    io::stdout().write_all(&output.stdout).unwrap();
    //println!("STDERR: ");
    //io::stdout().write_all(&output.stderr).unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout).replace("\r", "");
    let stdout = stdout.replace("Press any key to continue . . .", "");
    println!("{:?}", expected_output);
    println!("{:?}", stdout);
    expected_output.trim() == stdout.trim()
}

fn main() {
    let yaml = load_yaml!("cli.yml");
    //let matches = App::from(yaml).get_matches();
    run_gui(yaml, |matches| {
        let baseline = Path::new(matches.value_of("baseline").unwrap());
        let your_program = Path::new(matches.value_of("your program").unwrap());

        println!("{} {}", your_program.display(), baseline.display());

        if baseline.exists() && your_program.exists() {
            if let Ok(tests) = read_tests("tests.csv") {
                println!("{:?} {}", tests, tests.len());
                for (input, output) in tests {
                    if run_test(baseline, &input, &output) {
                        println!("Test successful!");
                    } else {
                        println!("Test failed...")
                    }
                }
            } else {
                println!("There is a problem. Make sure that tests.csv is in the folder.");
            }
        } else {
            println!("Make sure the paths point to an actual executable programs.");
        }
    });
    /*
    let baseline = Path::new(matches.value_of("baseline").unwrap());
    let your_program = Path::new(matches.value_of("your program").unwrap());

    println!("{} {}", your_program.display(), baseline.display());

    if let Ok(tests) = read_tests("tests.csv") {
        println!("{:?} {}", tests, tests.len());
        for (input, output) in tests {
            if run_test(baseline, &input, &output) {
                println!("Test successful!");
            } else {
                println!("Test failed...")
            }
        }
    } else {
        println!("There is a problem. Make sure that tests.csv is in the folder.");
    }
    */
}
