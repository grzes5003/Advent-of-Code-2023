import util/import_util as imp
import gleam/option as opt
import gleam/regex as re
import gleam/iterator as it
import gleam/dict
import gleam/string as str
import gleam/io

type Dict =
  dict.Dict(String, #(String, String))

fn parse(from line: String) -> #(String, #(String, String)) {
  let assert Ok(rgx) = re.from_string("^(.{3})\\s=\\s\\((.{3}),\\s(.{3})\\)$")
  let res = re.scan(rgx, line)
  let assert [re.Match(_, [opt.Some(a), opt.Some(b), opt.Some(c)])] = res
  #(a, #(b, c))
}

fn traverse(
  map: Dict,
  current: String,
  target: String,
  dirs: it.Iterator(String),
  steps: Int,
) -> Result(Int, Nil) {
  case current {
    t if t == target -> Ok(steps)
    _ -> {
      let assert Ok(next) = it.first(dirs)
      let assert Ok(#(l, r)) = dict.get(map, current)
      case next {
        "L" -> traverse(map, l, target, it.drop(dirs, 1), steps + 1)
        "R" -> traverse(map, r, target, it.drop(dirs, 1), steps + 1)
        _ -> panic
      }
    }
  }
}

fn input() -> List(String) {
  imp.read_text("day08")
  |> imp.split_lines()
}

fn solve1(map: Dict, dir: String) {
  let directions =
    str.split(dir, on: "")
    |> it.from_list()
    |> it.cycle()
  traverse(map, "AAA", "ZZZ", directions, 0)
}

pub fn main() {
  let assert [directions, _, ..rest] = input()
  let coords =
    it.from_list(rest)
    |> it.map(parse)
    |> it.to_list()
    |> dict.from_list()
  io.debug(solve1(coords, directions))
}
