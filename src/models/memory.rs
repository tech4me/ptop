use crate::system_monitor;
use crate::update::UpdateableWidget;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Style, Stylize},
    symbols::Marker,
    widgets::{Axis, Block, Borders, Chart, Dataset, GraphType, Widget},
};

pub struct Memory {
    memory_usage: Vec<(f64, f64)>,
    swap_usage: Vec<(f64, f64)>,
    update_count: usize,
}

impl Memory {
    pub fn new() -> Self {
        Memory {
            memory_usage: Vec::new(),
            swap_usage: Vec::new(),
            update_count: 0,
        }
    }
}

impl Widget for &Memory {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let datasets = vec![
            Dataset::default()
                .name("Memory Usage(%)")
                .marker(Marker::Braille)
                .graph_type(GraphType::Line)
                .style(Style::default().yellow())
                .data(&self.memory_usage),
            Dataset::default()
                .name("Swap Usage(%)")
                .marker(Marker::Braille)
                .graph_type(GraphType::Line)
                .style(Style::default().magenta())
                .data(&self.swap_usage),
        ];

        let x_axis = Axis::default()
            .style(Style::default().white())
            .bounds([self.update_count as f64 - 100.0, self.update_count as f64]);

        let y_axis = Axis::default()
            .style(Style::default().white())
            .bounds([0.0, 100.0]);

        let chart = Chart::new(datasets)
            .block(
                Block::new()
                    .borders(Borders::ALL)
                    .title("Memory Histogram")
                    .style(Style::default()),
            )
            .x_axis(x_axis)
            .y_axis(y_axis);

        chart.render(area, buf);
    }
}

impl UpdateableWidget for Memory {
    fn update(&mut self, monitor: &system_monitor::Monitor) {
        self.update_count += 1;
        self.memory_usage.push((
            self.update_count as f64,
            monitor.get_used_memory().0 as f64 / monitor.get_total_memory().0 as f64 * 100.0,
        ));
        self.swap_usage.push((
            self.update_count as f64,
            monitor.get_used_memory().1 as f64 / monitor.get_total_memory().1 as f64 * 100.0,
        ));
    }
}
