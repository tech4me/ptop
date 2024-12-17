use crate::app::{AppState, PTop};
use crate::system_monitor;

pub trait UpdateableWidget {
    fn update(&mut self, monitor: &system_monitor::Monitor);
}

pub trait UpdateableWidgetWithState {
    fn update_with_state(&mut self, monitor: &system_monitor::Monitor, state: &mut AppState);
}

pub fn update_widgets(app: &mut PTop, monitor: &system_monitor::Monitor) {
    app.widgets.cpu.update(monitor);
    app.widgets.info.update(monitor);
    app.widgets.memory.update(monitor);
    app.widgets.cpu_per_core.update(monitor);
    app.widgets
        .process_table
        .update_with_state(monitor, &mut app.state);
    app.widgets
        .alert_table
        .update_with_state(monitor, &mut app.state);
}
