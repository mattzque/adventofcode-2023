use std::{collections::HashMap, fs::read_to_string};

#[derive(Debug, PartialEq, Eq, Hash)]
enum Color {
    Red, Blue, Green
}

impl Color {
    fn from_string(string: &str) -> anyhow::Result<Self> {
        match string {
            "red" => Ok(Self::Red),
            "blue" => Ok(Self::Blue),
            "green" => Ok(Self::Green),
            _ => Err(anyhow::anyhow!("Invalid color!"))
        }
    }
}

#[derive(Default, Debug)]
struct Draw(HashMap<Color, usize>);

#[derive(Default, Debug)]
struct Game {
    id: usize,
    draws: Vec<Draw>,
}

fn parse_draw(draw: &str) -> anyhow::Result<Draw> {
    let mut colors = HashMap::new();
    for cube in draw.split(',') {
        let (count, color) = cube.trim().split_once(' ').ok_or(anyhow::anyhow!("Invalid Draw!"))?;
        let count = str::parse::<usize>(count).map_err(|_| anyhow::anyhow!("Invalid Draw Count!"))?;
        let color = Color::from_string(color)?;
        colors.insert(color, count);
    }

    Ok(Draw(colors))
}

fn parse_game(line: &str) -> anyhow::Result<Game> {
    let (game, rest) = line.split_once(": ").ok_or(anyhow::anyhow!("Invalid game!"))?;
    let id = str::parse::<usize>(&game["Game ".len()..]).map_err(|_| anyhow::anyhow!("Invalid Game Id!"))?;
    let mut draws = Vec::new();

    for draw in rest.split(';').map(parse_draw) {
        match draw {
            Ok(draw) => draws.push(draw),
            Err(err) => return Err(err)
        }
    }

    Ok(Game { id, draws })
}

fn parse_games_from_contents(contents: &str) -> anyhow::Result<Vec<Game>> {
    let lines = contents.split_terminator('\n');
    let mut games = Vec::new();

    for game in lines.map(parse_game) {
        match game {
            Ok(game) => games.push(game),
            Err(err) => return Err(err)
        }
    }

    Ok(games)
}

fn filter_games_by_min_count<'a>(games: &'a [Game], filter: &'a HashMap<Color, usize>) -> impl Iterator<Item = &'a Game> {
    games.iter().filter(|game| {
        game.draws.iter().all(|draw| {
            let Draw(cubes) = draw;
            cubes.iter().all(|(color, count)| {
                filter.get(color).unwrap() >= count
            })
        })
    })
}

fn main() {
    let contents = read_to_string("day2-input.txt").expect("Invalid Input!");
    let games = parse_games_from_contents(&contents).expect("Invalid Input!");
    let filter = HashMap::from([
        (Color::Red, 12),
        (Color::Green, 13),
        (Color::Blue, 14),
    ]);
    let sum_of_ids: usize = filter_games_by_min_count(&games, &filter).map(|game| game.id).sum();

    println!("Sum of IDs: {}", sum_of_ids);
}
