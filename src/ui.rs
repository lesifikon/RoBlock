use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Wrap},
    symbols::border,
    Frame,
};

use crate::app::App;

pub fn render(app: &mut App, frame: &mut Frame) {
    let chunks_horizontal = Layout::default()
        .direction(Direction::Horizontal)
        .vertical_margin(3) // 3
        .constraints([
            Constraint::Fill(1),
            Constraint::Length(54),
            Constraint::Fill(1),
            Constraint::Fill(1),
        ])
        .split(frame.area());

    let chunks_vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(30),
            Constraint::Percentage(70),
        ])
        .split(chunks_horizontal[2]);

    let title_block = create_block(" Game ");
    let title_block1 = create_block(" Next block ");
    let title_block2 = create_block("");

    // let mut my_grid = [[0; 10]; 20];
    // let colors = [Color::Red, Color::Green, Color::Blue, Color::Yellow, Color::Magenta];


    let col = Layout::default()
        .direction(Direction::Horizontal)
        .margin(2)
        // .vertical_margin(5)
        .constraints([
            Constraint::Fill(1); 10
        ])
        .split(chunks_horizontal[1]);

    let tteexxtt = title_text(title_block2.clone(), "");

    // println!("{}",app.fall_block[2][0]);

    for (col_idx, col_rect) in col.iter().enumerate() {
    let row = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(2); 20])
        .split(*col_rect);

    for (row_idx, row_rect) in row.iter().enumerate() {
        // let color = colors[(row_idx + col_idx) % colors.len()];
        let color = if app.matrix[row_idx][col_idx] == 0 { Color::Green } else { Color::Yellow };
        let block;

        if app.matrix[row_idx][col_idx] == 1 {
            block = Block::default().style(Style::default().bg(color));
        } else {
            block = Block::default().style(Style::default());
        }
        frame.render_widget(block, *row_rect);
    }
}
    let meni_title = title_text(title_block, "");
    let text = title_text(title_block1, "выфвфывфвфывфы");
    let textt = title_text(title_block2, "выфвфыв");


    frame.render_widget(meni_title, chunks_horizontal[1]);
    frame.render_widget(text, chunks_vertical[0]);
    frame.render_widget(textt, chunks_vertical[1]);
}

fn title_text<'a, T>(title_block: Block<'a>, text: T) -> Paragraph<'a> 
where
    T: Into<Text<'a>>, 
{
    let title = Paragraph::new(text)
        .style(Style::default().fg(Color::Green))
        .block(title_block);
    title
}

fn create_block(text: &str ) -> Block<'_> {
    let title = Block::default()
    .title(Line::from(text))
    .borders(Borders::ALL)
    .border_style(Style::default().fg(Color::Yellow))
    .style(Style::default());
    title
}
