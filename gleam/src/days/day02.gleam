import gleam/io
import util/import_util as imp
import gleam/string as str
import gleam/result
import gleam/list
import gleam/iterator as it
import gleam/int

type Cube {
  Cube(color: String, num: Int)
}

fn get_cube(lst: List(Cube), color: String) -> Result(Cube, Nil) {
  list.find(lst, fn(item) { item.color == color })
}

fn cube_from_str(input: String) -> Result(Cube, Nil) {
  let trimed_input = str.trim(input)
  use split <- result.try(str.split_once(trimed_input, on: " "))
  let #(num, name) = split
  use num <- result.try(int.parse(num))
  let known_colors = ["red", "green", "blue"]
  let contains = list.contains(known_colors, name)
  case name {
    n if contains -> Ok(Cube(n, num))
    _ -> Error(Nil)
  }
}

fn parse_cubes(input: List(String)) -> List(Cube) {
  parse_cubes_sup(input, [])
}

fn parse_cubes_sup(input: List(String), list: List(Cube)) -> List(Cube) {
  case input {
    [first, ..rest] -> {
      let assert Ok(bag) = cube_from_str(first)
      parse_cubes_sup(rest, [bag, ..list])
    }
    _ -> list
  }
}

pub type Bag {
  Bag(red: Int, green: Int, blue: Int)
}

fn bag_from_str(input: String) -> Result(Bag, Nil) {
  let cubes = str.split(input, on: ",")
  let cubes = parse_cubes(cubes)
  let red = result.unwrap(get_cube(cubes, "red"), Cube("red", 0)).num
  let blue = result.unwrap(get_cube(cubes, "blue"), Cube("blue", 0)).num
  let green = result.unwrap(get_cube(cubes, "green"), Cube("green", 0)).num
  Ok(Bag(red: red, blue: blue, green: green))
}

pub type Game {
  Game(id: Int, bags: List(Bag))
}

fn sum_game(game: Game) -> Bag {
  it.from_list(game.bags)
  |> it.fold(Bag(0, 0, 0), fn(acc, bag) {
    Bag(
      red: acc.red + bag.red,
      green: acc.green + bag.green,
      blue: acc.blue + bag.blue,
    )
  })
}

fn from_line(line: String) -> Result(Game, Nil) {
  use var <- result.try(str.split_once(line, on: ":"))
  use v2 <- result.try(str.split_once(var.0, on: " "))
  use game_id <- result.try(int.parse(
    v2.1
    |> str.trim,
  ))
  let bags_raw = str.split(var.1, on: ";")
  let #(bags, errs) =
    it.from_list(bags_raw)
    |> it.map(fn(bag_raw) { bag_from_str(bag_raw) })
    |> it.to_list()
    |> result.partition
  case errs {
    [] -> Ok(Game(id: game_id, bags: bags))
    _ -> Error(Nil)
  }
}

fn input() -> List(String) {
  imp.read_text("day02")
  |> imp.split_lines()
}

fn solve1(games: List(Game)) -> Int {
  let limits = Bag(red: 12, green: 13, blue: 14)
  it.from_list(games)
  |> it.filter(fn(game) {
    let sum = sum_game(game)
    sum.red <= limits.red
    && sum.blue <= limits.blue
    && sum.green <= limits.green
  })
  |> it.to_list
  |> list.length
}

pub fn main() {
  let #(games, _) =
    it.from_list(input())
    |> it.map(from_line)
    |> it.to_list
    |> result.partition
  io.debug(solve1(games))
}
