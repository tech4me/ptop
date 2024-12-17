use crate::system_monitor;
use crate::update::UpdateableWidget;
use humantime::format_duration;
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::Style,
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget},
};
use std::time::Duration;

pub struct Info {
    host_name: String,
    os_name: String,
    kernel_version: String,
    uptime: u64,
    cpu_name: String,
}

impl Info {
    pub fn new() -> Self {
        Info {
            host_name: "".to_string(),
            os_name: "".to_string(),
            kernel_version: "".to_string(),
            uptime: 0,
            cpu_name: "".to_string(),
        }
    }
}

impl Widget for &Info {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let paragraph = Paragraph::new(vec![
            Line::from(vec![Span::from("Host Name: "), Span::from(&self.host_name)]),
            Line::from(vec![Span::from("OS Name: "), Span::from(&self.os_name)]),
            Line::from(vec![
                Span::from("Kernel Version: "),
                Span::from(&self.kernel_version),
            ]),
            Line::from(vec![
                Span::from("Uptime: "),
                Span::from(format_duration(Duration::from_secs(self.uptime)).to_string()),
            ]),
            Line::from(vec![Span::from("CPU Name: "), Span::from(&self.cpu_name)]),
        ])
        .alignment(Alignment::Center)
        .block(
            Block::new()
                .borders(Borders::ALL)
                .title("System Info")
                .style(Style::default()),
        );
        paragraph.render(area, buf);
    }
}

impl UpdateableWidget for Info {
    fn update(&mut self, monitor: &system_monitor::Monitor) {
        self.host_name = monitor.get_host_name();
        self.os_name = monitor.get_os_name();
        self.kernel_version = monitor.get_kernel_version();
        self.uptime = monitor.get_uptime();
        self.cpu_name = monitor.get_cpu_name();
    }
}
