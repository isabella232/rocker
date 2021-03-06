use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;

use log::LevelFilter;
use termion::event::{Event, Key};
use tui::{
    layout::Rect,
    style::{Color, Style},
    widgets::Widget,
    Frame,
};
use tui_logger::{Dispatcher, EventListener, TuiLoggerSmartWidget, TuiWidgetState};

use crate::app::AppCommand;
use crate::docker::DockerExecutor;
use crate::views::View;
use crate::Backend;

pub struct AppLogsView {
    state: RefCell<TuiWidgetState>,
    dispatcher: Rc<RefCell<Dispatcher<Event>>>,
}

impl AppLogsView {
    pub fn new() -> AppLogsView {
        let state = TuiWidgetState::new();
        state.set_level_for_target("rkr", LevelFilter::Info);
        AppLogsView {
            state: RefCell::new(state),
            dispatcher: Rc::new(RefCell::new(Dispatcher::new())),
        }
    }
}

impl View for AppLogsView {
    fn handle_input(&mut self, key: Key, _docker: Arc<DockerExecutor>) -> Option<AppCommand> {
        if self.dispatcher.borrow_mut().dispatch(&Event::Key(key)) {
            Some(AppCommand::NoOp)
        } else {
            None
        }
    }

    fn refresh(&mut self, _docker: Arc<DockerExecutor>) {}

    fn draw(&self, t: &mut Frame<Backend>, rect: Rect) {
        TuiLoggerSmartWidget::default()
            .state(&*self.state.borrow_mut())
            .dispatcher(self.dispatcher.clone())
            .style_error(Style::default().fg(Color::Red))
            .style_debug(Style::default().fg(Color::Green))
            .style_warn(Style::default().fg(Color::Yellow))
            .style_trace(Style::default().fg(Color::Magenta))
            .style_info(Style::default().fg(Color::Cyan))
            .render(t, rect);
    }
}
