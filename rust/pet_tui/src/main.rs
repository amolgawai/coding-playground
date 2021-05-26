use chrono::prelude::*;
use crossterm::{event::{self, Event as CEvent, KeyCode, KeyEvent}, terminal::{disable_raw_mode, enable_raw_mode}};
use petname;
use rand::{self, Rng};
use serde::{Deserialize, Serialize};
use std::{fs, io::Stdout};
use std::io;
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};
use thiserror::Error;
use tui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{
        Block, BorderType, Borders, Cell, List, ListItem, ListState, Paragraph, Row, Table, Tabs,
    },
    Terminal,
};

const DB_PATH: &str = "./data/db.json";

#[derive(Serialize, Deserialize, Clone)]
struct Pet {
    id: usize,
    name: String,
    category: String,
    age: usize,
    created_at: DateTime<Utc>,
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("Error reading db file: {0}")]
    ReadDBError(#[from] std::io::Error),
    #[error("Error parsing db file: {0}")]
    ParseDBError(#[from] serde_json::Error),
}

enum Event<I> {
    Input(I),
    Tick,
}

#[derive(Copy, Clone, Debug)]
enum MenuItem {
    Home,
    Pets,
}
impl From<MenuItem> for usize {
    fn from(input: MenuItem) -> usize {
        match input {
            MenuItem::Home => 0,
            MenuItem::Pets => 1,
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    run_app()
}

fn run_app() -> Result<(), Box<dyn std::error::Error>> {

    enable_raw_mode().expect("can run in raw mode");

    let (tx, rx) = mpsc::channel();
    thread::spawn(move || events_loop(tx));

    let terminal = setup_terminal()?;
    rendering_loop(rx, terminal)
}

fn events_loop(tx: mpsc::Sender<Event<KeyEvent>>) {

    let tick_rate = Duration::from_millis(200);
    let mut last_tick = Instant::now();
    loop {
        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if event::poll(timeout).expect("poll works") {
            if let CEvent::Key(key) = event::read().expect("can read events") {
                tx.send(Event::Input(key)).expect("can send events");
            }
        }

        if last_tick.elapsed() >= tick_rate {
            if let Ok(_) = tx.send(Event::Tick) {
                last_tick = Instant::now();
            }
        }
    }
}

fn setup_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>, Box<dyn std::error::Error>> {
    // setup an terminal to draw the app
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;
    Ok(terminal)
}

fn rendering_loop(rx: mpsc::Receiver<Event<KeyEvent>>, mut terminal: Terminal<CrosstermBackend<Stdout>>) -> Result<(), Box<dyn std::error::Error>> {

    // setup rendering loop
    let mut active_menu_item = MenuItem::Home;
    let mut pet_list_state = ListState::default();
    pet_list_state.select(Some(0));

    loop {
        terminal.draw(|rect| {
            let size = rect.size();
            let chunks = layout_chunks(&size);

            let copyright = copyright_para();
            rect.render_widget(copyright, chunks[2]);

            let tabs = menu_tabs(active_menu_item);
            rect.render_widget(tabs, chunks[0]);

            match active_menu_item {
                MenuItem::Home => rect.render_widget(render_home("Home"), chunks[1]),
                MenuItem::Pets => {
                    let pets_chunks = Layout::default()
                        .direction(Direction::Horizontal)
                        .constraints(
                            [Constraint::Percentage(20), Constraint::Percentage(80)].as_ref(),
                        )
                        .split(chunks[1]);

                    let pet_list = read_db().expect("can fetch pet list");
                    if !pet_list.is_empty() {
                        let (left, right) = render_pets(&pet_list_state, &pet_list);
                        rect.render_stateful_widget(left, pets_chunks[0], &mut pet_list_state);
                        rect.render_widget(right, pets_chunks[1]);
                    } else {
                        rect.render_widget(render_home("Pets"), chunks[1])
                    }
                }
            }
        })?;

        match rx.recv()? {
            Event::Input(event) => match event.code {
                KeyCode::Char('q') => {
                    disable_raw_mode()?;
                    terminal.show_cursor()?;
                    break;
                }
                KeyCode::Char('h') => active_menu_item = MenuItem::Home,
                KeyCode::Char('p') => active_menu_item = MenuItem::Pets,
                KeyCode::Char('a') => {
                    add_random_pet_to_db().expect("can add new random pet");
                }
                KeyCode::Char('d') => {
                    remove_pet_at_index(&mut pet_list_state).expect("can remove pet");
                }
                KeyCode::Down => {
                    if let Some(selected) = pet_list_state.selected() {
                        let amount_pets = read_db().expect("can fetch pet list").len();
                        if selected >= amount_pets - 1 {
                            pet_list_state.select(Some(0));
                        } else {
                            pet_list_state.select(Some(selected + 1));
                        }
                    }
                }
                KeyCode::Up => {
                    if let Some(selected) = pet_list_state.selected() {
                        let amount_pets = read_db().expect("can fetch pet list").len();
                        if selected > 0 {
                            pet_list_state.select(Some(selected - 1));
                        } else {
                            pet_list_state.select(Some(amount_pets - 1));
                        }
                    }
                }
                _ => {}
            },
            Event::Tick => {}
        }
    }

    Ok(())
}

fn layout_chunks(size: &tui::layout::Rect) -> Vec<tui::layout::Rect> {
    Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Min(2),
                Constraint::Length(3),
            ]
            .as_ref(),
        )
        .split(*size)
}

fn copyright_para() -> Paragraph<'static> {
    Paragraph::new("pet-CLI 2021 - all rights reserved")
        .style(Style::default().fg(Color::LightCyan))
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White))
                .title("Copyright")
                .border_type(BorderType::Plain),
        )
}

fn menu_tabs(active_menu_item: MenuItem) -> Tabs<'static> {
    let menu_titles = vec!["Home", "Pets", "Add", "Delete", "Quit"];
    let menu = menu_titles
        .iter()
        .map(|t| {
            let (first, rest) = t.split_at(1);
            Spans::from(vec![
                Span::styled(
                    first,
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::UNDERLINED),
                ),
                Span::styled(rest, Style::default().fg(Color::White)),
            ])
        })
        .collect();

    Tabs::new(menu)
        .select(active_menu_item.into())
        .block(Block::default().title("Menu").borders(Borders::ALL))
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().fg(Color::Yellow))
        .divider(Span::raw("|"))
}

fn render_home<'a>(title: &'a str) -> Paragraph<'a> {
    let home = Paragraph::new(vec![
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw("Welcome")]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw("to")]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::styled(
            "pet-CLI",
            Style::default().fg(Color::LightBlue),
        )]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw("Press 'p' to access pets, 'a' to add random new pets and 'd' to delete the currently selected pet.")]),
    ])
    .alignment(Alignment::Center)
    .block(
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::White))
            .title(title)
            .border_type(BorderType::Plain),
    );
    home
}

fn read_db() -> Result<Vec<Pet>, Error> {
    let db_content = get_db_contents();
    // let mut parsed: Vec<Pet> = Vec::new();
    let parsed = if !db_content.is_empty() {
        serde_json::from_str(&db_content)?
    } else {
        add_random_pet_to_db()?
    };
    Ok(parsed)
}

fn render_pets<'a>(pet_list_state: &ListState, pet_list: &Vec<Pet>) -> (List<'a>, Table<'a>) {
    let pets = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::White))
        .title("Pets")
        .border_type(BorderType::Plain);

    let items: Vec<_> = pet_list
        .iter()
        .map(|pet| {
            ListItem::new(Spans::from(vec![Span::styled(
                pet.name.clone(),
                Style::default(),
            )]))
        })
        .collect();

    let selected_pet = pet_list
        .get(
            pet_list_state
                .selected()
                .expect("there is always a selected pet"),
        )
        .expect("exists")
        .clone();

    let list = List::new(items).block(pets).highlight_style(
        Style::default()
            .bg(Color::Yellow)
            .fg(Color::Black)
            .add_modifier(Modifier::BOLD),
    );

    let pet_detail = Table::new(vec![Row::new(vec![
        Cell::from(Span::raw(selected_pet.id.to_string())),
        Cell::from(Span::raw(selected_pet.name)),
        Cell::from(Span::raw(selected_pet.category)),
        Cell::from(Span::raw(selected_pet.age.to_string())),
        Cell::from(Span::raw(selected_pet.created_at.to_string())),
    ])])
    .header(Row::new(vec![
        Cell::from(Span::styled(
            "ID",
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Cell::from(Span::styled(
            "Name",
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Cell::from(Span::styled(
            "Category",
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Cell::from(Span::styled(
            "Age",
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Cell::from(Span::styled(
            "Created At",
            Style::default().add_modifier(Modifier::BOLD),
        )),
    ]))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::White))
            .title("Detail")
            .border_type(BorderType::Plain),
    )
    .widths(&[
        Constraint::Percentage(5),
        Constraint::Percentage(20),
        Constraint::Percentage(20),
        Constraint::Percentage(5),
        Constraint::Percentage(20),
    ]);

    (list, pet_detail)
}

fn add_random_pet_to_db() -> Result<Vec<Pet>, Error> {
    let mut rng = rand::thread_rng();
    let db_content = get_db_contents();
    let mut parsed: Vec<Pet> = Vec::new();
    if !db_content.is_empty() {
        parsed = serde_json::from_str(&db_content)?;
    }
    let catsdogs = match rng.gen_range(0..2) {
        0 => "cats",
        _ => "dogs",
    };
    let random_name = petname::Petnames::default().generate_one(10, "-");
    let pet_name = random_name.split('-').next_back().unwrap_or_default();
    let random_pet = Pet {
        id: rng.gen_range(0..10000000),
        // name: rng.sample_iter(Alphanumeric).take(10).collect(),
        // name: std::iter::repeat(())
        // .map(|()| rng.sample(Alphanumeric))
        // .map(char::from)
        // .take(10)
        // .collect(),
        name: pet_name.to_owned(),
        category: catsdogs.to_owned(),
        age: rng.gen_range(1..16),
        created_at: Utc::now(),
    };

    parsed.push(random_pet);
    fs::write(DB_PATH, &serde_json::to_vec(&parsed)?)?;
    Ok(parsed)
}

fn remove_pet_at_index(pet_list_state: &mut ListState) -> Result<(), Error> {
    if let Some(selected) = pet_list_state.selected() {
        let db_content = fs::read_to_string(DB_PATH)?;
        let mut parsed: Vec<Pet> = serde_json::from_str(&db_content)?;
        parsed.remove(selected);
        fs::write(DB_PATH, &serde_json::to_vec(&parsed)?)?;
        if parsed.is_empty() {
            pet_list_state.select(None);
        } else {
            if selected != 0 {
                pet_list_state.select(Some(selected - 1));
            }
        }
    }
    Ok(())
}

fn get_db_contents() -> String {
    let db_path = std::path::Path::new(DB_PATH);
    let mut db_content = String::new();

    if db_path.exists() {
        db_content = fs::read_to_string(DB_PATH).expect("no database found");
    }
    db_content
}
