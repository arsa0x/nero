use tabled::{
    Table, Tabled,
    settings::{Alignment, Color, Modify, object::Rows, style::Style},
};

#[derive(Tabled)]
pub struct TableSummary {
    pub label: String,
    pub method: String,
    pub status: String,
    pub time: String,
    pub size: String,
}

pub struct OutputPrint;

impl OutputPrint {
    pub fn table_summary(data: Vec<crate::cmds::run::RunCmd>) {
        let data_table: Vec<TableSummary> = data
            .into_iter()
            .map(|f| {
                let time_str = format!("{} ms", f.duration_ms);
                let size_str = if f.size >= 1024 {
                    format!("{:.2} KB", f.size as f64 / 1024.0)
                } else {
                    format!("{} B", f.size)
                };

                TableSummary {
                    label: f.label,
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

        println!("\nHASIL ANALISIS REQUEST:");
        println!("{}", table);
    }
}
