trait GameState {
    fn apply(data: &mut Game);
    fn render_frame(data: &mut Game);
}

