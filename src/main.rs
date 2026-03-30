use crate::models::LineOutput;
use crate::state::AppState;
use std::path::Path;

mod models;
mod state;

fn main() {
    // Process args.
    let file_path_str = std::env::args()
        .nth(1)
        .expect("Expected a file path as the first argument");

    // Initiate app state.
    let mut app_state = AppState::new();

    // Open the file a process each line.
    let file_path = Path::new(file_path_str.as_str());
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .trim(csv::Trim::All)
        .from_path(file_path)
        .expect("Failed to open csv file");

    for (i, result) in reader.deserialize().enumerate() {
        let transaction = result.unwrap_or_else(|error| {
            panic!(
                "Failed to deserialize line {i} from csv file. Error: {}",
                error
            )
        });

        app_state.process_transaction(transaction);
    }

    // Emit the result.
    let mut writer = csv::Writer::from_writer(std::io::stdout());

    for client_account in app_state.client_accounts.values() {
        let line_output = LineOutput::from_account(client_account);

        writer
            .serialize(line_output)
            .expect("Failed to serialize line");
    }

    writer.flush().expect("Failed to flush writer");
}
