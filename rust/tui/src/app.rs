use crate::util::{RandomSignal, SinSignal, StatefulList, TabsState};

const TASKS: [&str; 24] = [
    "Item1", "Item2", "Item3", "Item4", "Item5", "Item6", "Item7", "Item8", "Item9", "Item10",
    "Item11", "Item12", "Item13", "Item14", "Item15", "Item16", "Item17", "Item18", "Item19",
    "Item20", "Item21", "Item22", "Item23", "Item24",
];

const LOGS: [(&str, &str); 26] = [
    ("Event1", "INFO"),
    ("Event2", "INFO"),
    ("Event3", "CRITICAL"),
    ("Event4", "ERROR"),
    ("Event5", "INFO"),
    ("Event6", "INFO"),
    ("Event7", "WARNING"),
    ("Event8", "INFO"),
    ("Event9", "INFO"),
    ("Event10", "INFO"),
    ("Event11", "CRITICAL"),
    ("Event12", "INFO"),
    ("Event13", "INFO"),
    ("Event14", "INFO"),
    ("Event15", "INFO"),
    ("Event16", "INFO"),
    ("Event17", "ERROR"),
    ("Event18", "ERROR"),
    ("Event19", "INFO"),
    ("Event20", "INFO"),
    ("Event21", "WARNING"),
    ("Event22", "INFO"),
    ("Event23", "INFO"),
    ("Event24", "WARNING"),
    ("Event25", "INFO"),
    ("Event26", "INFO"),
];

const EVENTS: [(&str, u64); 24] = [
    ("B1", 9),
    ("B2", 12),
    ("B3", 5),
    ("B4", 8),
    ("B5", 2),
    ("B6", 4),
    ("B7", 5),
    ("B8", 9),
    ("B9", 14),
    ("B10", 15),
    ("B11", 1),
    ("B12", 0),
    ("B13", 4),
    ("B14", 6),
    ("B15", 4),
    ("B16", 6),
    ("B17", 4),
    ("B18", 7),
    ("B19", 13),
    ("B20", 8),
    ("B21", 11),
    ("B22", 9),
    ("B23", 3),
    ("B24", 5),
];

pub struct Signal<S: Iterator> {
    source: S,
    pub points: Vec<S::Item>,
    tick_rate: usize,
}

impl<S> Signal<S>
where
    S: Iterator,
{
    fn on_tick(&mut self) {
        for _ in 0..self.tick_rate {
            self.points.remove(0);
        }
        self.points
            .extend(self.source.by_ref().take(self.tick_rate));
    }
}

pub struct Server<'a> {
    pub name: &'a str,
    pub location: &'a str,
    pub coords: (f64, f64),
    pub status: &'a str,
}

pub struct App<'a> {
    pub title: &'a str,
    pub should_quit: bool,
    pub tabs: TabsState<'a>,
    pub text: String,
    pub text_line_count: usize,
    pub show_chart: bool,
    pub progress: f64,
    pub tasks: StatefulList<&'a str>,
    pub logs: StatefulList<(&'a str, &'a str)>,
    pub enhanced_graphics: bool,
    pub row_top: usize,
}

impl<'a> App<'a> {
    pub fn new(title: &'a str, enhanced_graphics: bool) -> App<'a> {
        App {
            title,
            should_quit: false,
            tabs: TabsState::new(vec!["Editor", "Terminal"]),
            text: String::new(),
            text_line_count: 0,
            show_chart: true,
            progress: 0.0,
            tasks: StatefulList::with_items(TASKS.to_vec()),
            logs: StatefulList::with_items(LOGS.to_vec()),
            enhanced_graphics,
            row_top: 0,
        }
    }

    pub fn on_open_file(&mut self) {
        self.text = std::fs::read_to_string("src/app.rs").expect("read file");
        self.text_line_count = self.text.lines().count();
    }

    pub fn on_page_up(&mut self) {
        if self.row_top > 0 {
            self.row_top -= 1;
        }
    }

    pub fn on_page_down(&mut self) {
        if self.row_top < self.text_line_count {
            self.row_top += 1;
        }
    }

    pub fn on_up(&mut self) {
        if self.row_top > 0 {
            self.row_top -= 1;
        }
    }

    pub fn on_down(&mut self) {
        if self.row_top < self.text_line_count {
            self.row_top += 1;
        }
    }

    pub fn on_right(&mut self) {
        self.tabs.next();
    }

    pub fn on_select_editor_tab(&mut self) {
        self.tabs.index = 0;
    }

    pub fn on_select_terminal_tab(&mut self) {
        self.tabs.index = 1;
    }

    pub fn on_left(&mut self) {
        self.tabs.previous();
    }

    pub fn on_key(&mut self, c: char) {
        match c {
            'q' => {
                self.should_quit = true;
            }
            't' => {
                self.show_chart = !self.show_chart;
            }
            _ => {}
        }
    }

    pub fn on_tick(&mut self) {
        // Update progress
        self.progress += 0.001;
        if self.progress > 1.0 {
            self.progress = 0.0;
        }
    }
}
