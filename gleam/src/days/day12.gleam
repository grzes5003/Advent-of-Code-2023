import util/import_util as imp
import gleam/iterator as it
import gleam/string as str
import gleam/otp/task
import gleam/result
import gleam/list
import gleam/int
import gleam/io

type Status {
  Operational
  Damaged
}

fn print_list(of: List(Status)) -> String {
  of
  |> it.from_list()
  |> it.map(fn(status) {
    case status {
      Operational -> "."
      Damaged -> "#"
    }
  })
  |> it.to_list()
  |> str.join("")
}

fn verify(input: List(Status), req: List(Int)) -> Bool {
  let chunks =
    input
    |> it.from_list()
    |> it.chunk(fn(item) { item == Operational })
    |> it.filter(fn(ls) { list.contains(ls, Damaged) })
    |> it.to_list()
  let match =
    chunks
    |> it.from_list()
    |> it.zip(it.from_list(req))
    |> it.all(fn(pair) {
      let #(iter, req) = pair
      let len =
        iter
        |> list.length()
      len == req
    })
  match && list.length(chunks) == list.length(req)
}

fn append_buff(buff: List(List(Status))) -> List(List(Status)) {
  case buff {
    [] -> [[Operational], [Damaged]]
    _ -> {
      list.append(append_buff2(buff, Operational), append_buff2(buff, Damaged))
    }
  }
}

fn append_buff2(buff: List(List(Status)), op: Status) -> List(List(Status)) {
  case buff {
    [] -> [[op]]
    _ ->
      buff
      |> it.from_list()
      |> it.map(fn(item) { list.append(item, [op]) })
      |> it.to_list()
  }
}

fn generate(input: List(String), buff: List(List(Status))) -> List(List(Status)) {
  case input {
    [] -> buff
    [next, ..rest] -> {
      case next {
        "." -> {
          append_buff2(buff, Operational)
          |> generate(rest, _)
        }
        "#" -> {
          append_buff2(buff, Damaged)
          |> generate(rest, _)
        }
        "?" -> {
          append_buff(buff)
          |> generate(rest, _)
        }
        _ -> panic
      }
    }
  }
}

fn input() -> List(String) {
  imp.read_text("day12")
  |> imp.split_lines()
}

pub fn parse_line(line: String) -> #(List(String), List(Int)) {
  let assert Ok(#(l, r)) = str.split_once(line, on: " ")
  let assert #(req, []) =
    str.split(r, ",")
    |> it.from_list()
    |> it.map(int.parse)
    |> it.to_list()
    |> result.partition()
  #(str.split(l, ""), list.reverse(req))
}

pub fn handle(input: List(String), req: List(Int)) -> Int {
  let possible = generate(input, [])
  possible
  |> it.from_list()
  |> it.map(fn(poss) { verify(poss, req) })
  |> it.filter(fn(val) { val })
  |> it.to_list
  |> list.length
}

pub fn solve1(lines: List(#(List(String), List(Int)))) -> Int {
  lines
  |> it.from_list()
  |> it.map(fn(item) {
    let #(input, req) = item
    task.async(fn() { handle(input, req) })
  })
  |> it.map(task.await_forever)
  |> it.fold(0, fn(acc, val) { acc + val })
}

pub fn main() {
  let input =
    input()
    |> it.from_list()
    |> it.map(parse_line)
    |> it.to_list()
  io.debug(solve1(input))
}
