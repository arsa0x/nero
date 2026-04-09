use colored::Colorize;
use tabled::{
    Table, Tabled,
    derive::display::truncate,
    settings::{Alignment, Color, Modify, object::Rows, style::Style},
};

#[derive(Tabled)]
pub struct PrintTable {
    pub label: String,
    pub path: String,
    pub method: String,
    pub status: String,
    pub time: String,
    pub size: String,
}

pub struct OutputPrint;

impl OutputPrint {
    fn format_size(size: u64) -> String {
        if size >= 1024 * 1024 {
            format!("{:.2} MB", size as f64 / (1024.0 * 1024.0))
        } else if size >= 1024 {
            format!("{:.2} KB", size as f64 / 1024.0)
        } else {
            format!("{} B", size)
        }
    }

    // fn color_status(status: u16) -> String {
    //     match status {
    //         200..=299 => status.to_string().green().bold().to_string(),
    //         300..=399 => status.to_string().yellow().bold().to_string(),
    //         _ => status.to_string().red().bold().to_string(),
    //     }
    // }

    pub fn json(data: &Vec<crate::cmds::run::RunCmd>) {
        match serde_json::to_string_pretty(data) {
            Ok(json) => println!("{}", json),
            Err(e) => eprintln!("Failed to serialize JSON: {}", e),
        }
    }

    pub fn table(data: &Vec<crate::cmds::run::RunCmd>) {
        if data.is_empty() {
            println!("No request executed");
            return;
        }

        println!("File: {}", data[0].file.green().bold());
        println!("Time: {}\n", data[0].date.green());

        let data_table: Vec<PrintTable> = data
            .iter()
            .map(|f| PrintTable {
                path: truncate(&f.path, 20),
                label: f.label.clone(),
                method: f.method.to_uppercase(),
                status: f.status.to_string(),
                time: format!("{} ms", f.duration_ms),
                size: Self::format_size(f.size),
            })
            .collect();

        let mut table = Table::new(data_table);

        table
            .with(Style::modern())
            .with(
                Modify::new(Rows::first())
                    .with(Color::FG_CYAN)
                    .with(Alignment::center()),
            )
            .with(Modify::new(Rows::new(1..)).with(Alignment::left()));

        println!("{}", table);

        let total_time: u128 = data.iter().map(|d| d.duration_ms).sum();
        let success = data.iter().filter(|d| d.status < 400).count();
        let failed = data.len() - success;

        println!(
            "\nTotal: {}  Success: {}  Failed: {}  Duration: {} ms",
            data.len().to_string().bold(),
            success.to_string().green(),
            failed.to_string().red(),
            total_time.to_string().yellow()
        );
    }

    pub fn summary(data: &Vec<crate::cmds::run::RunCmd>) {
        for (i, f) in data.iter().enumerate() {
            let status_label = if f.status < 400 {
                "OK".green().bold()
            } else {
                "FAIL".red().bold()
            };

            println!(
                "[{}/{}] Running: {}... {} ({} ms)",
                i + 1,
                data.len(),
                f.label.cyan(),
                status_label,
                f.duration_ms
            );
        }

        let success = data.iter().filter(|d| d.status < 400).count();
        let failed = data.len() - success;

        println!();

        if failed == 0 {
            println!("{} All requests passed ({})", "✔".green(), success);
        } else {
            println!("{} {} passed, {} failed", "✖".red(), success, failed);
        }
    }
}
