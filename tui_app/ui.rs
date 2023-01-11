pub fn ui<B: Backend>(f: &mut Frame<B>, app: &App) {
    // Wrapping block for a group
    // Just draw the block and the group on the same area and build the group
    // with at least a margin of 1
    let size = f.size();

    // Surrounding block
    let block = Block::default()
        .borders(Borders::ALL)
        .title("Test App")
        .title_alignment(Alignment::Center)
        .border_type(BorderType::Plain);
    f.render_widget(block, size);

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(2)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(f.size());

    // Top two inner blocks
    let left_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(chunks[0]);

    // Top left inner block with green background
    let block = Block::default()
        .border_style(Style::default().fg(Color::Magenta))
        .borders(Borders::ALL)
        .border_type(BorderType::Plain)
        .title_alignment(Alignment::Left)
        .title("Test Box")
        .style(Style::default());
    f.render_widget(block, left_chunks[0]);

    let block = Block::default()
        .border_style(Style::default().fg(Color::Magenta))
        .borders(Borders::ALL)
        .border_type(BorderType::Plain)
        .title("Test Box")
        .title_alignment(Alignment::Left);
    f.render_widget(block, left_chunks[1]);

    // Bottom two inner blocks
    let right_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(100)].as_ref())
        .split(chunks[1]);

    let block = Block::default()
        .border_style(Style::default().fg(Color::Magenta))
        .borders(Borders::ALL)
        .border_type(BorderType::Plain)
        .title("Test Box")
        .title_alignment(Alignment::Left);
    f.render_widget(block, right_chunks[0]);
    // Bottom right block with styled left and right border
}
