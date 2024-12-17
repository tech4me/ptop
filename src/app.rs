use crate::models::alert;
use crate::models::cpu;
use crate::models::info;
use crate::models::memory;
use crate::models::process;
use ratatui::{
    layout::{Constraint, Direction, Flex, Layout},
    prelude::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph},
    DefaultTerminal,
};
use std::io;

pub struct PTop {
    terminal: DefaultTerminal,
    pub widgets: Widgets,
    pub state: AppState,
}

pub enum Mode {
    Normal,
    ProcessFilter,
    AlertCpuThreshold,
    AlertMemoryThreshold,
}

pub struct AppState {
    pub mode: Mode,
    pub filter: String,
    pub alert_cpu_threshold: String,
    pub alert_memory_threshold: String,
    pub process_table_state: process::ProcessTableState,
    pub alert_table_state: alert::AlertTableState,
}

impl AppState {
    pub fn new() -> Self {
        AppState {
            mode: Mode::Normal,
            filter: String::new(),
            alert_cpu_threshold: String::new(),
            alert_memory_threshold: String::new(),
            process_table_state: process::ProcessTableState::new(),
            alert_table_state: alert::AlertTableState::new(),
        }
    }

    pub fn select_prev_process(&mut self) {
        self.process_table_state.select_prev();
    }

    pub fn select_next_process(&mut self) {
        self.process_table_state.select_next();
    }

    pub fn select_prev_alert(&mut self) {
        self.alert_table_state.select_prev();
    }

    pub fn select_next_alert(&mut self) {
        self.alert_table_state.select_next();
    }
}

pub struct Widgets {
    pub cpu: cpu::CpuHistogram,
    pub info: info::Info,
    pub memory: memory::Memory,
    pub cpu_per_core: cpu::CpuPerCore,
    pub process_table: process::ProcessTable,
    pub alert_table: alert::AlertTable,
}

impl PTop {
    pub fn new() -> Self {
        PTop {
            terminal: ratatui::init(),
            widgets: Widgets {
                cpu: cpu::CpuHistogram::new(),
                info: info::Info::new(),
                memory: memory::Memory::new(),
                cpu_per_core: cpu::CpuPerCore::new(),
                process_table: process::ProcessTable::new(),
                alert_table: alert::AlertTable::new(),
            },
            state: AppState::new(),
        }
    }

    pub fn draw(&mut self) -> Result<(), io::Error> {
        self.terminal.draw(|f| {
            // Overall Layout
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
                .split(f.area());

            // Top half layout
            let top_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(40), Constraint::Percentage(60)].as_ref())
                .split(chunks[0]);

            // Bottom half layout
            let bottom_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(40), Constraint::Percentage(60)].as_ref())
                .split(chunks[1]);

            // Bottom-left nested layout
            let bottom_left_chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints(
                    [
                        Constraint::Percentage(30),
                        Constraint::Percentage(30),
                        Constraint::Percentage(40),
                    ]
                    .as_ref(),
                )
                .split(bottom_chunks[0]);

            // Draw blocks for all sections
            let block_style = Style::default().fg(Color::White).bg(Color::Black);

            // Top-left: System Info
            f.render_widget(&self.widgets.info, top_chunks[0]);

            // Top-right: Alert Table
            f.render_stateful_widget(
                &self.widgets.alert_table,
                top_chunks[1],
                &mut self.state.alert_table_state.state,
            );

            // Bottom-left top: CPU Histogram
            f.render_widget(&self.widgets.cpu, bottom_left_chunks[0]);

            // Bottom-left bottom: Memory Histogram
            f.render_widget(&self.widgets.memory, bottom_left_chunks[1]);

            // Bottom-left middle: CPU Per Core
            f.render_widget(&self.widgets.cpu_per_core, bottom_left_chunks[2]);

            // Bottom-right bottom: Process Table
            f.render_stateful_widget(
                &self.widgets.process_table,
                bottom_chunks[1],
                &mut self.state.process_table_state.state,
            );

            // Popup
            let popup_area = |area: Rect, percent_x: u16, percent_y: u16| {
                let vertical =
                    Layout::vertical([Constraint::Percentage(percent_y)]).flex(Flex::Center);
                let horizontal =
                    Layout::horizontal([Constraint::Percentage(percent_x)]).flex(Flex::Center);
                let [area] = vertical.areas(area);
                let [area] = horizontal.areas(area);
                area
            };
            let area = popup_area(f.area(), 20, 10);
            match self.state.mode {
                Mode::ProcessFilter => {
                    let popup =
                        Paragraph::new(vec![Line::from(vec![Span::from(&self.state.filter)])])
                            .block(
                                Block::new()
                                    .borders(Borders::ALL)
                                    .title("Process Filter")
                                    .style(block_style),
                            );
                    f.render_widget(Clear, area);
                    f.render_widget(popup, area);
                }
                Mode::AlertCpuThreshold => {
                    let popup = Paragraph::new(vec![Line::from(vec![Span::from(
                        &self.state.alert_cpu_threshold,
                    )])])
                    .block(
                        Block::new()
                            .borders(Borders::ALL)
                            .title("Alert CPU Threshold")
                            .style(block_style),
                    );
                    f.render_widget(Clear, area);
                    f.render_widget(popup, area);
                }
                Mode::AlertMemoryThreshold => {
                    let popup = Paragraph::new(vec![Line::from(vec![Span::from(
                        &self.state.alert_memory_threshold,
                    )])])
                    .block(
                        Block::new()
                            .borders(Borders::ALL)
                            .title("Alert Memory Threshold")
                            .style(block_style),
                    );
                    f.render_widget(Clear, area);
                    f.render_widget(popup, area);
                }
                _ => {}
            }
        })?;
        Ok(())
    }

    pub fn finish(&mut self) {
        ratatui::restore();
    }
}
