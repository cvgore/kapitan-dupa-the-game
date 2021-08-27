struct Game {
    data: GameData,
    state: dyn GameState,
}

struct GameData {
    life_count: u8,
    game_over: bool,
    score: u32,
}
