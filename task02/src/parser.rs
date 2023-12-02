type ID = u32;
#[derive(Debug, Clone)]
pub struct Game {
    pub id: u32,
    pub sets: Vec<Set>,
}

impl Game {
    pub fn new(id: u32, sets: Vec<Set>) -> Self {
        Self { id, sets }
    }
}

#[derive(Debug, Clone)]
pub struct Set {
    pub red: u32,
    pub green: u32,
    pub blue: u32,
}

impl Set {
    pub fn new_empty() -> Self {
        Self {
            red: 0,
            green: 0,
            blue: 0,
        }
    }
    pub fn add_color_and_count(&mut self, count: u32, color: &str) {
        match color {
            "red" => self.red = count,
            "green" => self.green = count,
            "blue" => self.blue = count,
            _ => panic!("unexpected color token! {color}"),
        }
    }

    /// Creates a new Set that contains the maximal values of each color quantity.
    pub fn eat_max(&self, next: &Set) -> Self {
        Self {
            red: self.red.max(next.red),
            green: self.green.max(next.green),
            blue: self.blue.max(next.blue),
        }
    }
}

pub fn parse_input(input: &str) -> Vec<Game> {
    input.lines().map(parse_line).collect()
}

const GAME_STR_LEN: usize = "Game".len();

/// Each line is formatted like this:
/// Game GAME_ID COLON SET_1 SEMICOLON SET_2 SEMICOLON ... SET_N NEW LINE
/// Each set is formatted as repetitions of the following schema:
/// [NUMBER COLOR_NAME COMMA]
/// Sets are separated from each other by semicolons. The last set does not end with a semicolon.
fn parse_line(line: &str) -> Game {
    let line = line.trim(); // <- to accommodate for bad raw string format during testing
    let (id, colon_pos) = get_id(line);

    let all_sets_raw = line[colon_pos + 1..].trim();
    let mut all_sets_raw = all_sets_raw.split(';');

    let mut sets = Vec::new();
    while let Some(raw_set) = all_sets_raw.next() {
        let set = get_set(raw_set);
        sets.push(set);
    }

    Game::new(id, sets)
}

/// Cut off line at first colon, throw away the colon. Walk to the left of it,
/// throw away everything after the first whitespace.
fn get_id(line: &str) -> (ID, usize) {
    let colon_pos = line.find(':').unwrap();
    (
        line[GAME_STR_LEN..colon_pos].trim().parse().unwrap(),
        colon_pos,
    )
}

/// Get each pair of tokens that are formatted like this: [NUMBER WHITESPACE COLOR] separated by a comma each.
fn get_set(raw_set: &str) -> Set {
    let mut quantities = raw_set.split(',');
    let mut set = Set::new_empty();
    while let Some(count_and_color) = quantities.next() {
        let count_and_color = count_and_color.trim();
        let mut count_and_color = count_and_color.split(' ');
        let count = count_and_color
            .next()
            .expect("digit is there")
            .parse()
            .unwrap();
        let color = count_and_color.next().expect("color is there");
        set.add_color_and_count(count, color);
    }
    set
}
