use ratatui::crossterm::event;
use update::update_widgets;

mod app;
mod models;
mod system_monitor;
mod update;

fn main() {
    let mut app = app::PTop::new();
    let mut monitor = system_monitor::Monitor::new();

    loop {
        monitor.update();
        update_widgets(&mut app, &monitor);
        if let Err(e) = app.draw() {
            eprintln!("Failed to draw UI: {}", e);
            return;
        }

        if event::poll(std::time::Duration::from_millis(1000)).unwrap() {
            if let event::Event::Key(key) = event::read().unwrap() {
                match app.state.mode {
                    app::Mode::Normal => match key.code {
                        event::KeyCode::Char('q') => break,
                        event::KeyCode::Char('j') => app.state.select_next_process(),
                        event::KeyCode::Char('k') => app.state.select_prev_process(),
                        event::KeyCode::Char('t') => {
                            if let Some(selected) = app.state.process_table_state.selected() {
                                app.widgets
                                    .process_table
                                    .terminate_process(&mut monitor, selected);
                            }
                        }
                        event::KeyCode::Char('/') => {
                            // Clear the current filter when entering a new filter
                            app.state.filter.clear();
                            app.state.mode = app::Mode::ProcessFilter;
                        }
                        event::KeyCode::Char('1') => {
                            app.state.process_table_state.sort_condition =
                                models::process::SortBy::Pid;
                            app.state.process_table_state.sort_ascending =
                                !app.state.process_table_state.sort_ascending;
                        }
                        event::KeyCode::Char('2') => {
                            app.state.process_table_state.sort_condition =
                                models::process::SortBy::Name;
                            app.state.process_table_state.sort_ascending =
                                !app.state.process_table_state.sort_ascending;
                        }
                        event::KeyCode::Char('3') => {
                            app.state.process_table_state.sort_condition =
                                models::process::SortBy::CpuUsage;
                            app.state.process_table_state.sort_ascending =
                                !app.state.process_table_state.sort_ascending;
                        }
                        event::KeyCode::Char('4') => {
                            app.state.process_table_state.sort_condition =
                                models::process::SortBy::Memory;
                            app.state.process_table_state.sort_ascending =
                                !app.state.process_table_state.sort_ascending;
                        }
                        event::KeyCode::Char('5') => {
                            app.state.process_table_state.sort_condition =
                                models::process::SortBy::RunTime;
                            app.state.process_table_state.sort_ascending =
                                !app.state.process_table_state.sort_ascending;
                        }
                        event::KeyCode::Char('6') => {
                            app.state.process_table_state.sort_condition =
                                models::process::SortBy::Status;
                            app.state.process_table_state.sort_ascending =
                                !app.state.process_table_state.sort_ascending;
                        }
                        event::KeyCode::Up => app.state.select_prev_alert(),
                        event::KeyCode::Down => app.state.select_next_alert(),
                        event::KeyCode::Char('a') => {
                            if let Some(selected) = app.state.alert_table_state.selected() {
                                app.widgets.alert_table.arm_alert(selected);
                            }
                        }
                        event::KeyCode::Char('d') => {
                            if let Some(selected) = app.state.alert_table_state.selected() {
                                app.widgets.alert_table.disarm_alert(selected);
                            }
                        }
                        event::KeyCode::Char('c') => {
                            app.state.alert_cpu_threshold.clear();
                            app.state.mode = app::Mode::AlertCpuThreshold;
                        }
                        event::KeyCode::Char('m') => {
                            app.state.alert_memory_threshold.clear();
                            app.state.mode = app::Mode::AlertMemoryThreshold;
                        }
                        event::KeyCode::Char('e') => {
                            if let Some(selected) = app.state.process_table_state.selected() {
                                let process = app.widgets.process_table.get_process(selected);
                                app.widgets
                                    .alert_table
                                    .add_exit_code_alert(process.pid, process.name.clone());
                                app.state.alert_table_state.select_next();
                            }
                        }
                        _ => {}
                    },
                    app::Mode::ProcessFilter => match key.code {
                        event::KeyCode::Enter => {
                            app.state.process_table_state.filter = app.state.filter.clone();
                            app.state.mode = app::Mode::Normal;
                        }
                        event::KeyCode::Backspace => {
                            app.state.filter.pop();
                        }
                        event::KeyCode::Char(c) => {
                            app.state.filter.push(c);
                        }
                        _ => {}
                    },
                    app::Mode::AlertCpuThreshold => match key.code {
                        event::KeyCode::Enter => {
                            if let Some(selected) = app.state.process_table_state.selected() {
                                let process = app.widgets.process_table.get_process(selected);
                                app.widgets.alert_table.add_cpu_alert(
                                    process.pid,
                                    process.name.clone(),
                                    app.state.alert_cpu_threshold.parse().unwrap(),
                                );
                                app.state.alert_table_state.select_next();
                            }
                            app.state.mode = app::Mode::Normal;
                        }
                        event::KeyCode::Backspace => {
                            app.state.alert_cpu_threshold.pop();
                        }
                        event::KeyCode::Char(c) => {
                            app.state.alert_cpu_threshold.push(c);
                        }
                        _ => {}
                    },
                    app::Mode::AlertMemoryThreshold => match key.code {
                        event::KeyCode::Enter => {
                            if let Some(selected) = app.state.process_table_state.selected() {
                                let process = app.widgets.process_table.get_process(selected);
                                app.widgets.alert_table.add_memory_alert(
                                    process.pid,
                                    process.name.clone(),
                                    app.state.alert_memory_threshold.parse().unwrap(),
                                );
                                app.state.alert_table_state.select_next();
                            }
                            app.state.mode = app::Mode::Normal;
                        }
                        event::KeyCode::Backspace => {
                            app.state.alert_memory_threshold.pop();
                        }
                        event::KeyCode::Char(c) => {
                            app.state.alert_memory_threshold.push(c);
                        }
                        _ => {}
                    },
                }
            }
        }
    }
    app.finish();
}
