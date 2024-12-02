use color_eyre::Result;
use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEvent},
    layout::{Alignment, Constraint, Layout, Rect},
    style::{palette::tailwind, Color, Style, Stylize},
    text::Text,
    DefaultTerminal, Frame,
};
use ratatui::{prelude::*, widgets::*};
use std::time::{Duration, Instant};

fn main() -> Result<()> {
    let map = vec![
        vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
        vec![1, 2, 0, 0, 0, 0, 0, 1, 0, 1],
        vec![1, 1, 0, 1, 0, 1, 0, 1, 0, 1],
        vec![1, 0, 1, 0, 0, 1, 0, 0, 0, 1],
        vec![1, 0, 0, 0, 1, 0, 0, 1, 1, 1],
        vec![1, 0, 1, 0, 0, 1, 0, 0, 0, 1],
        vec![1, 0, 0, 1, 0, 0, 1, 0, 1, 1],
        vec![1, 1, 0, 0, 0, 1, 1, 0, 0, 1],
        vec![1, 0, 0, 1, 0, 0, 0, 0, 3, 1],
        vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
    ];
    color_eyre::install()?;
    let terminal = ratatui::init();
    let app_result = App::new(&map).run(terminal);
    ratatui::restore();
    app_result
}

// 定义 App 结构体
struct App {
    exit: bool,
    map: Vec<Vec<i32>>,
}

impl App {
    // 新建一个 App 实例
    pub fn new(map: &Vec<Vec<i32>>) -> Self {
        App {
            exit: false,
            map: map.clone(),
        }
    }

    // 运行方法
    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        let tick_rate = Duration::from_millis(250);
        let mut last_tick = Instant::now();

        while !self.exit {
            terminal.draw(|f| self.drawWidget(f))?;
            let timeout = tick_rate.saturating_sub(last_tick.elapsed());
            if event::poll(timeout)? {
                if let Event::Key(event) = event::read()? {
                    self.handle_input(event);
                }
            }
            if last_tick.elapsed() >= tick_rate {
                last_tick = Instant::now();
            }
        }
        Ok(())
    }

    // 处理输入
    fn handle_input(&mut self, event: KeyEvent) {
        match event.code {
            KeyCode::Char('q') => self.exit = true,
            _ => {}
        }
    }

    fn calculate_layout(&self, area: Rect) -> (Rect, Vec<Vec<Rect>>, Rect) {
        let main_layout = Layout::vertical([
            Constraint::Length(1),
            Constraint::Min(0),
            Constraint::Length(3),
        ]);
        let block_layout = Layout::vertical([Constraint::Percentage(100)]);
        let [title_area, main_area, footer_area] = main_layout.areas(area);
        let mut main_areas: Vec<Vec<Rect>> = block_layout
            .split(main_area)
            .iter()
            .map(|&area| {
                Layout::horizontal([Constraint::Percentage(25), Constraint::Percentage(75)])
                    .split(area)
                    .to_vec()
            })
            .collect();
    
        let vertical_layout = Layout::vertical([Constraint::Percentage(25), Constraint::Percentage(75)]);
            let first_part = (&mut main_areas)[0][0];
            let split_first_part = vertical_layout.split(first_part).to_vec();
            (&mut main_areas)[0][0] = split_first_part[0];
            (&mut main_areas)[0].insert(1, split_first_part[1]);
        
    
        (title_area, main_areas, footer_area)
    }

    fn drawWidget(&self, frame: &mut Frame) {
        let (title_area, layout, footer_area) = self.calculate_layout(frame.area());

        self.render_title(frame, title_area);

        let paragraph = self.placeholder_paragraph();

        self.render_border_descript(BorderType::Rounded, frame, layout[0][0]);
        self.render_border_options(&paragraph, BorderType::Rounded, frame, layout[0][1]);
        self.render_border_map(BorderType::Rounded, frame, layout[0][2]);

        self.render_footer(frame, footer_area);
    }

    fn render_title(&self, frame: &mut Frame, area: Rect) {
        frame.render_widget(
            Paragraph::new("Welcome to DUNGEONS. Press q to quit")
                .dark_gray()
                .alignment(Alignment::Center),
            area,
        );
    }

    fn render_footer(&self, frame: &mut Frame, area: Rect) {
        const INFO_TEXT: [&str; 1] = [
            "(q) quit | (↑) previous option | (↓) next option | (Enter) select option | (←) previous path | (→) next path",
        ];

        let info_footer = Paragraph::new(Text::from_iter(INFO_TEXT))
            .style(
                Style::new()
                    .fg(tailwind::SLATE.c200)
                    .bg(tailwind::SLATE.c800),
            )
            .centered()
            .block(
                Block::bordered()
                    .border_type(BorderType::Double)
                    .border_style(Style::new().fg(tailwind::SLATE.c200)),
            );
        frame.render_widget(info_footer, area);
    }

    fn placeholder_paragraph(&self) -> Paragraph<'static> {
        let text = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.";
        Paragraph::new(text.dark_gray()).wrap(Wrap { trim: true })
    }

    fn render_border_options(
        &self,
        paragraph: &Paragraph,
        border_type: BorderType,
        frame: &mut Frame,
        area: Rect,
    ) {
        let block = Block::bordered()
            .border_type(border_type)
            .padding(Padding::new(5, 10, 1, 2))
            .title(format!("Options"));
        frame.render_widget(paragraph.clone().block(block), area);
    }

    fn render_border_descript(
        &self,
        border_type: BorderType,
        frame: &mut Frame,
        area: Rect,
    ) {
        let block = Block::bordered()
            .border_type(border_type)
            .padding(Padding::new(2, 5, 1, 2))
            .title(format!("Description"));
        let text = vec![
            Line::from(vec!["Minimal steps: ".into(), "NULL".green().bold().into()]),
            Line::from(vec!["  Total paths: ".into(), "NULL".green().bold().into()]),
        ];
        frame.render_widget(Paragraph::new(text).block(block), area);
    }

    fn render_border_map(
        &self,
        border_type: BorderType,
        frame: &mut Frame,
        area: Rect,
    ) {
        let block = Block::bordered()
            .border_type(border_type)
            .padding(Padding::new(5, 10, 1, 2))
            .title(format!("Map"));
        let mut map_lines = vec![];
        for line in self.map.iter() {
            let mut map_line = vec![];
            for &cell in line.iter() {
                let cell_text = match cell {
                    0 => "  ".into(),
                    1 => "██".into(),
                    2 => "A ".into(),
                    3 => "B ".into(),
                    _ => "  ".into(),
                };
                map_line.push(cell_text);
            }
            map_lines.push(Line::from(map_line));
        }

        frame.render_widget(Paragraph::new(map_lines).block(block), area);
    }
}
