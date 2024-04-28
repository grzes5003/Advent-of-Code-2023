import util/import_util as imp
import gleam/result
import gleam/string
import gleam/iterator as it
import gleam/io
import gleam/int

fn is_integer(string: String) -> Bool {
  case int.parse(string) {
    Ok(_) -> True
    _ -> False
  }
}

fn find_num(word: String) -> String {
  string.split(word, "")
  |> it.from_list()
  |> it.filter(keeping: is_integer)
  |> it.first
  |> result.unwrap("")
}

fn extract_digits(word: String) -> Int {
  let res =
    { find_num(word) <> find_num(string.reverse(word)) }
    |> int.parse()
  case res {
    Ok(r) -> r
    _ -> panic
  }
}

fn solve1(lines: List(String)) -> Int {
  it.from_list(lines)
  |> it.map(extract_digits)
  |> it.reduce(fn(acc, x) { acc + x })
  |> result.unwrap(0)
}

fn input() -> List(String) {
  imp.read_text("day01")
  |> imp.split_lines()
}

pub fn main() -> Nil {
  let input = input()
  io.debug(solve1(input))
  Nil
}
