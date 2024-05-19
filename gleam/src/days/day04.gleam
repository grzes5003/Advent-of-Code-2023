import util/import_util as imp
import gleam/io
import gleam/int
import gleam/result
import gleam/string as str
import gleam/iterator as it
import gleam/set

pub type Card {
  Card(winning: set.Set(Int), guesses: List(Int))
}

fn card(from line: String) -> Result(Card, Nil) {
  use split <- result.try(str.split_once(line, on: "|"))
  let #(left, right) = split
  // Left side handling
  use split <- result.try(str.split_once(left, on: ":"))
  let #(_, winning) = split
  let assert #(winning, []) =
    str.trim(winning)
    |> str.split(on: " ")
    |> it.from_list()
    |> it.filter(fn(s) { s != "" })
    |> it.map(int.parse)
    |> it.to_list()
    |> result.partition()
  // Right side handling
  let assert #(guesses, []) =
    str.trim(right)
    |> str.split(on: " ")
    |> it.from_list()
    |> it.filter(fn(s) { s != "" })
    |> it.map(int.parse)
    |> it.to_list()
    |> result.partition()
  Ok(Card(winning: set.from_list(winning), guesses: guesses))
}

fn count_wins(card: Card) -> Int {
  card.guesses
  |> it.from_list
  |> it.fold(from: 0, with: fn(acc, el) {
    case set.contains(card.winning, el) {
      True -> {
        case acc {
          0 -> 1
          val -> val * 2
        }
      }
      False -> acc
    }
  })
}

fn input() -> List(String) {
  imp.read_text("day04")
  |> imp.split_lines()
}

fn solve1(cards: List(Card)) -> Int {
  cards
  |> it.from_list
  |> it.map(count_wins)
  |> it.fold(0, fn(acc, el) {acc + el})
}

pub fn main() {
  let #(cards, _) =
    it.from_list(input())
    |> it.map(card)
    |> it.to_list
    |> result.partition
  io.debug(solve1(cards))
}
