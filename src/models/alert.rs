use crate::update::UpdateableWidgetWithState;
use crate::{app::AppState, system_monitor};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Rect},
    style::{Color, Style, Stylize},
    widgets::{Block, Borders, Row, StatefulWidget, Table, TableState},
};

#[derive(PartialEq, strum::EnumString, strum::Display)]
pub enum AlertStatus {
    Armed,
    Disarmed,
    Triggered,
}

#[derive(PartialEq, strum::EnumString, strum::Display)]
pub enum AlertCondition {
    CpuUsage(f32),
    MemoryUsage(f32),
    Exit(),
}

pub struct AlertEntry {
    pid: u32,
    name: String,
    condition: AlertCondition,
    status: AlertStatus,
}

pub struct AlertTable {
    alerts: Vec<AlertEntry>,
}

impl AlertTable {
    pub fn new() -> Self {
        AlertTable { alerts: Vec::new() }
    }

    pub fn add_cpu_alert(&mut self, pid: u32, name: String, threshold: f32) {
        self.alerts.push(AlertEntry {
            pid,
            name,
            condition: AlertCondition::CpuUsage(threshold),
            status: AlertStatus::Armed,
        });
    }

    pub fn add_memory_alert(&mut self, pid: u32, name: String, threshold: f32) {
        self.alerts.push(AlertEntry {
            pid,
            name,
            condition: AlertCondition::MemoryUsage(threshold),
            status: AlertStatus::Armed,
        });
    }

    pub fn add_exit_code_alert(&mut self, pid: u32, name: String) {
        self.alerts.push(AlertEntry {
            pid,
            name,
            condition: AlertCondition::Exit(),
            status: AlertStatus::Armed,
        });
    }

    pub fn arm_alert(&mut self, index: usize) {
        if index < self.alerts.len() {
            self.alerts[index].status = AlertStatus::Armed;
        }
    }

    pub fn disarm_alert(&mut self, index: usize) {
        if index < self.alerts.len() {
            self.alerts[index].status = AlertStatus::Disarmed;
        }
    }

    pub fn update_alerts(&mut self, monitor: &system_monitor::Monitor) {
        for alert in self.alerts.iter_mut() {
            if alert.status == AlertStatus::Armed {
                if let Some((cpu_usage, memory_usage, status)) =
                    monitor.get_process_status_by_pid(alert.pid)
                {
                    match alert.condition {
                        AlertCondition::CpuUsage(threshold) => {
                            if cpu_usage > threshold {
                                alert.status = AlertStatus::Triggered;
                            }
                        }
                        AlertCondition::MemoryUsage(threshold) => {
                            if memory_usage > threshold {
                                alert.status = AlertStatus::Triggered;
                            }
                        }
                        AlertCondition::Exit() => {
                            if status == sysinfo::ProcessStatus::Stop {
                                alert.status = AlertStatus::Triggered;
                            }
                        }
                    }
                } else {
                    // if the process is not found, we assume it has exited
                    if alert.condition == AlertCondition::Exit() {
                        alert.status = AlertStatus::Triggered;
                    }
                }
            }
        }
    }
}

pub struct AlertTableState {
    pub state: TableState,
}

impl AlertTableState {
    pub fn new() -> Self {
        AlertTableState {
            state: TableState::default().with_selected(Some(0)),
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

impl StatefulWidget for &AlertTable {
    type State = TableState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let rows = self
            .alerts
            .iter()
            .map(|a| {
                let status_style = match a.status {
                    AlertStatus::Armed => Style::default().fg(Color::Green),
                    AlertStatus::Disarmed => Style::default().fg(Color::Gray),
                    AlertStatus::Triggered => Style::default().fg(Color::Red),
                };

                Row::new(vec![
                    a.pid.to_string(),
                    a.name.clone(),
                    a.condition.to_string(),
                    a.status.to_string(),
                ])
                .style(status_style)
            })
            .collect::<Vec<Row>>();

        let header = Row::new(vec![
            "PID".to_string(),
            "Process Name".to_string(),
            "Condition".to_string(),
            "Status".to_string(),
        ]);

        let table = Table::new(
            rows,
            [
                Constraint::Percentage(10),
                Constraint::Percentage(50),
                Constraint::Percentage(20),
                Constraint::Percentage(20),
            ],
        )
        .block(
            Block::new()
                .borders(Borders::ALL)
                .title("Alerts")
                .style(Style::default()),
        )
        .header(header)
        .row_highlight_style(Style::new().bold());
        table.render(area, buf, state);
    }
}

impl UpdateableWidgetWithState for AlertTable {
    fn update_with_state(&mut self, monitor: &system_monitor::Monitor, _: &mut AppState) {
        self.update_alerts(monitor);
    }
}
