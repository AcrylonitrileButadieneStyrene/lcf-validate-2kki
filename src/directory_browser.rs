use crossterm::event::{Event, KeyModifiers};
use lcf::{ConvertExt as _, lmt::Map};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, List, ListState, Paragraph},
};

struct State {
    path: std::path::PathBuf,
    maps: Vec<(u32, Map)>,
    out: Option<std::path::PathBuf>,
    list: ListState,
}

pub fn run(path: &std::path::Path) -> std::path::PathBuf {
    let tree = lcf::lmt::LcfMapTree::read(&mut std::io::Cursor::new(
        std::fs::read(path.join("RPG_RT.lmt")).unwrap(),
    ))
    .unwrap();

    let mut tui = ratatui::init();
    let mut state = State {
        path: path.to_path_buf(),
        maps: tree.maps,
        out: None,
        list: ListState::default().with_selected(Some(0)),
    };
    loop {
        tui.draw(|frame| draw(frame, &mut state)).unwrap();
        match crossterm::event::read().unwrap() {
            Event::Key(e)
                if e.is_press() && e.modifiers == KeyModifiers::CONTROL && e.code.is_char('c') =>
            {
                ratatui::restore();
                std::process::exit(1);
            }
            Event::Key(e) if e.is_press() && e.code.is_up() => {
                let selected = state.list.selected().unwrap();
                state.list = state.list.with_selected(Some(selected.saturating_sub(1)));
            }
            Event::Key(e) if e.is_press() && e.code.is_down() => {
                let selected = state.list.selected().unwrap();
                state.list = state.list.with_selected(Some(selected + 1));
            }
            Event::Key(e) if e.is_press() && e.code.is_page_up() => {
                let selected = state.list.selected().unwrap();
                state.list = state.list.with_selected(Some(selected.saturating_sub(40)));
            }
            Event::Key(e) if e.is_press() && e.code.is_page_down() => {
                let selected = state.list.selected().unwrap();
                state.list = state.list.with_selected(Some(selected + 40));
            }
            Event::Key(e) if e.is_press() && e.code.is_enter() => {
                let id = state.maps[state.list.selected().unwrap()].0;
                state.out = Some(path.join(format!("Map{id:04}.lmu")));
            }
            _ => (),
        }

        if let Some(out) = state.out {
            ratatui::restore();
            return out;
        }
    }
}

fn draw(frame: &mut ratatui::Frame, state: &mut State) {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(1), Constraint::Percentage(100)].as_ref())
        .split(frame.area());

    frame.render_widget(Paragraph::new(state.path.to_string_lossy()), layout[0]);
    frame.render_stateful_widget(
        List::new(state.maps.iter().map(|(id, map)| {
            let name = encoding_rs::SHIFT_JIS.decode(&map.name).0;
            format!(
                "MAP{id:04}.lmu: {}{name}",
                " ".repeat(map.indentation as usize * 2)
            )
        }))
        .block(Block::new().borders(Borders::TOP))
        .highlight_style(Style::new().fg(Color::LightGreen))
        .highlight_symbol(">"),
        layout[1],
        &mut state.list,
    );
}
