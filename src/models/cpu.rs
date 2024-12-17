use crate::system_monitor;
use crate::update::UpdateableWidget;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Style, Stylize},
    symbols::Marker,
    widgets::{Axis, BarChart, Block, Borders, Chart, Dataset, GraphType, Widget},
};

pub struct CpuHistogram {
    usage: Vec<(f64, f64)>,
    update_count: usize,
}

impl CpuHistogram {
    pub fn new() -> Self {
        CpuHistogram {
            usage: Vec::new(),
            update_count: 0,
        }
    }
}

impl Widget for &CpuHistogram {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let datasets = vec![Dataset::default()
            .name("CPU Usage(%)")
            .marker(Marker::Braille)
            .graph_type(GraphType::Line)
            .style(Style::default().cyan())
            .data(&self.usage)];

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
                    .title("CPU Histogram")
                    .style(Style::default()),
            )
            .x_axis(x_axis)
            .y_axis(y_axis);

        chart.render(area, buf);
    }
}

impl UpdateableWidget for CpuHistogram {
    fn update(&mut self, monitor: &system_monitor::Monitor) {
        self.update_count += 1;
        self.usage.push((
            self.update_count as f64,
            monitor.get_global_cpu_usage() as f64,
        ));
    }
}

pub struct CpuPerCore {
    usage: Vec<f64>,
    update_count: usize,
}

impl CpuPerCore {
    pub fn new() -> Self {
        CpuPerCore {
            usage: Vec::new(),
            update_count: 0,
        }
    }
}

impl Widget for &CpuPerCore {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let data: Vec<(String, u64)> = self
            .usage
            .iter()
            .enumerate()
            .map(|(i, &usage)| (format!("Core {}", i), usage as u64))
            .collect();

        let bars: Vec<(&str, u64)> = data.iter().map(|(s, v)| (s.as_str(), *v)).collect();

        let bar_chart = BarChart::default()
            .block(
                Block::new()
                    .borders(Borders::ALL)
                    .title("CPU Per Core Usage")
                    .style(Style::default()),
            )
            .bar_width(8)
            .bar_gap(2)
            .bar_style(Style::default().green())
            .value_style(Style::default().white())
            .data(&bars)
            .max(100);

        bar_chart.render(area, buf);
    }
}

impl UpdateableWidget for CpuPerCore {
    fn update(&mut self, monitor: &system_monitor::Monitor) {
        self.update_count += 1;
        self.usage = monitor
            .get_per_cpu_usage()
            .iter()
            .map(|&usage| usage as f64)
            .collect();
    }
}
