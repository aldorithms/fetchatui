use ratatui::{
    layout::{Alignment, Constraint, Layout, Direction},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};
use sysinfo::{SystemExt, CpuExt};
use num_format::{Locale, ToFormattedString};


use crate::app::App;

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {

    let mut system = sysinfo::System::new_all();
    system.refresh_all();


    // This is where you add new widgets.
    // See the following resources:
    // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
    // - https://github.com/ratatui-org/ratatui/tree/master/examples
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints(
            [
                Constraint::Percentage(50),
                Constraint::Percentage(50),
            ].as_ref()
        )
        .split(frame.size());

    let icon_content = std::fs::read_to_string("icon.txt")
        .unwrap_or("".to_string());

    let system_information = format!(
        "
        {}@{}\n\
        -------------------------------------------------\n\
        OS: {} \n\
        Host: Unimplemented \n\
        Kernel: {} \n\
        Uptime: {}d, {}h, {}m, {}s \n\
        Shell: {} \n\
        DE: {} \n\
        CPU: {} - {}% \n\
        RAM:  {} / {} bytes - {}% \n\
        SWAP: {} / {} bytes - {}% \n\
        ",
        whoami::username(),
        whoami::hostname(),
        system.long_os_version().unwrap(),
        system.kernel_version().unwrap(),
        system.uptime().div_euclid(86400), // 86400 seconds in a day
        (system.uptime().div_euclid(3600)).rem_euclid(24), // 3600 seconds in an hour
        (system.uptime().div_euclid(60)).rem_euclid(60), // 60 seconds in a minute
        system.uptime().rem_euclid(60), // 1 second per minute
        query_shell::get_shell_name().unwrap(),
        whoami::desktop_env(),
        system.global_cpu_info().brand(), system.global_cpu_info().cpu_usage(),
        system.used_memory().to_formatted_string(&Locale::en),
        system.total_memory().to_formatted_string(&Locale::en),
        (system.used_memory() as f64 / system.total_memory() as f64) * 100.0,
        system.used_swap().to_formatted_string(&Locale::en),
        system.total_swap().to_formatted_string(&Locale::en),
        (system.used_swap() as f64 / system.total_swap() as f64) * 100.0,
        //disks_info.join("")
    );

    let left_block = Paragraph::new(icon_content)
        .block(Block::default().borders(Borders::ALL))
        .alignment(Alignment::Left);

    let right_block = Paragraph::new(system_information)
        .block(Block::default().title("Fetchatui").title_alignment(Alignment::Center).borders(Borders::ALL).border_type(BorderType::Rounded))
        .style(Style::default())
        .alignment(Alignment::Left);

    frame.render_widget(left_block, chunks[0]);
    frame.render_widget(right_block, chunks[1]);

}
