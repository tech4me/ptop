use crate::update::UpdateableWidgetWithState;
use crate::{app::AppState, system_monitor};
use humansize::{format_size, BINARY};
use humantime::format_duration;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Rect},
    style::{Style, Stylize},
    widgets::{Block, Borders, Row, StatefulWidget, Table, TableState},
};
use std::time::Duration;
use sysinfo::ProcessStatus;

#[derive(Clone, Copy, Debug)]
pub enum SortBy {
    Pid,
    Name,
    CpuUsage,
    Memory,
    RunTime,
    Status,
}

pub struct Process {
    pub pid: u32,
    pub name: String,
    cpu_usage: f32,
    memory: u64,
    run_time: u64,
    status: ProcessStatus,
}

impl Process {
    fn sort_by(&self, other: &Process, sort_by: SortBy) -> std::cmp::Ordering {
        match sort_by {
            SortBy::Pid => other.pid.cmp(&self.pid),
            SortBy::Name => other.name.cmp(&self.name),
            SortBy::CpuUsage => other
                .cpu_usage
                .partial_cmp(&self.cpu_usage)
                .unwrap_or(std::cmp::Ordering::Equal),
            SortBy::Memory => other.memory.cmp(&self.memory),
            SortBy::RunTime => other.run_time.cmp(&self.run_time),
            SortBy::Status => other.status.to_string().cmp(&self.status.to_string()),
        }
    }
}

pub struct ProcessTable {
    processes: Vec<Process>,
}

impl ProcessTable {
    pub fn new() -> Self {
        ProcessTable {
            processes: Vec::new(),
        }
    }

    pub fn sort_by(&mut self, sort_by: SortBy, sort_ascending: bool) {
        self.processes.sort_by(|a, b| {
            let order = a.sort_by(b, sort_by);
            if sort_ascending {
                order.reverse()
            } else {
                order
            }
        });
    }

    pub fn filter(&mut self, filter: &str) {
        let filter = filter.to_lowercase();
        self.processes
            .retain(|p| p.name.to_lowercase().contains(&filter));
    }

    pub fn terminate_process(&mut self, monitor: &mut system_monitor::Monitor, row: usize) {
        if row < self.processes.len() {
            let pid = self.processes[row].pid;
            monitor.terminate_process(pid);
        }
    }

    pub fn get_process(&self, row: usize) -> &Process {
        &self.processes[row]
    }
}

pub struct ProcessTableState {
    pub state: TableState,
    pub sort_condition: SortBy,
    pub sort_ascending: bool,
    pub filter: String,
}

impl ProcessTableState {
    pub fn new() -> Self {
        ProcessTableState {
            state: TableState::default().with_selected(Some(0)),
            sort_condition: SortBy::CpuUsage,
            sort_ascending: false,
            filter: String::new(),
        }
    }

    pub fn select_prev(&mut self) {
        self.state.select_previous();
    }

    pub fn select_next(&mut self) {
        self.state.select_next();
    }

    pub fn selected(&self) -> Option<usize> {
        self.state.selected()
    }
}

impl StatefulWidget for &ProcessTable {
    type State = TableState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let rows = self
            .processes
            .iter()
            .map(|p| {
                Row::new(vec![
                    p.pid.to_string(),
                    p.name.clone(),
                    p.cpu_usage.to_string() + "%",
                    format_size(p.memory, BINARY),
                    format_duration(Duration::from_secs(p.run_time)).to_string(),
                    p.status.to_string(),
                ])
            })
            .collect::<Vec<Row>>();

        let header = Row::new(vec![
            "PID".to_string(),
            "Name".to_string(),
            "CPU".to_string(),
            "Memory".to_string(),
            "Run Time".to_string(),
            "Status".to_string(),
        ]);

        let table = Table::new(
            rows,
            [
                Constraint::Percentage(10), // PID
                Constraint::Percentage(30), // Name
                Constraint::Percentage(10), // CPU
                Constraint::Percentage(10), // Memory
                Constraint::Percentage(30), // Run Time
                Constraint::Percentage(10), // Status
            ],
        )
        .block(
            Block::new()
                .borders(Borders::ALL)
                .title("Processes")
                .style(Style::default()),
        )
        .header(header)
        .row_highlight_style(Style::new().bold());
        table.render(area, buf, state);
    }
}

impl UpdateableWidgetWithState for ProcessTable {
    fn update_with_state(&mut self, monitor: &system_monitor::Monitor, state: &mut AppState) {
        self.processes = monitor
            .get_processes()
            .into_iter()
            .map(|(pid, name, cpu_usage, memory, run_time, status)| Process {
                pid,
                name,
                cpu_usage,
                memory,
                run_time,
                status,
            })
            .collect();
        self.filter(&state.process_table_state.filter);
        self.sort_by(
            state.process_table_state.sort_condition,
            state.process_table_state.sort_ascending,
        );
    }
}
