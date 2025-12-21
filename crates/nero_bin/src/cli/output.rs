use colored::Colorize;
use tabled::{
    Table, Tabled,
    settings::{Alignment, Color, Modify, object::Rows, style::Style},
};

#[derive(Tabled)]
pub struct PrintSummary {
    pub label: String,
    pub method: String,
    pub status: String,
    pub time: String,
    pub size: String,
}

#[derive(Tabled)]
pub struct PrintTable {
    pub label: String,
    pub method: String,
    pub status: String,
    pub time: String,
    pub size: String,
}

pub struct OutputPrint {
    // pub data: &'a Vec<crate::cmds::run::RunCmd>,
}

impl OutputPrint {
    // pub fn new(data: &Vec<crate::cmds::run::RunCmd>) -> Self {
    //     Self { data }
    // }
    pub fn json(data: &Vec<crate::cmds::run::RunCmd>) {}
    pub fn table(data: &Vec<crate::cmds::run::RunCmd>) {}
    pub fn summary(data: &Vec<crate::cmds::run::RunCmd>) {
        println!("File: {}", data[0].file.green());

        println!("Time: {}\n", data[0].date.green());

        // [1/2] Running: simple_get... OK
        // [2/2] Running: test_get_user... OK

        // ┌─────────────────┬────────┬────────┬────────┬──────┐
        // │      label      │ method │ status │  time  │ size │
        // ├─────────────────┼────────┼────────┼────────┼──────┤
        // │ simple_get      │ GET    │ 200    │ 1 ms   │ 12 B │
        // ├─────────────────┼────────┼────────┼────────┼──────┤
        // │ test_get_user   │ GET    │ 200    │ 0 ms   │ 8 B  │
        // └─────────────────┴────────┴────────┴────────┴──────┘
        let data_table: Vec<PrintSummary> = data
            .into_iter()
            .map(|f| {
                let time_str = format!("{} ms", f.duration_ms);
                let size_str = if f.size >= 1024 {
                    format!("{:.2} KB", f.size as f64 / 1024.0)
                } else {
                    format!("{} B", f.size)
                };

                PrintSummary {
                    label: f.label.clone(),
                    method: f.method.to_uppercase(),
                    status: f.status.to_string(),
                    time: time_str,
                    size: size_str,
                }
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
    }
}
