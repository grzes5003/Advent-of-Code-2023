import util/import_util as imp
import gleam/iterator as it
import gleam/string as str
import gleam/result
import gleam/otp/task
import gleam/int
import gleam/list
import gleam/io

fn extrapolate(list: List(Int), sum: Int) -> Int {
  let zeros = list.all(list, fn(x) { x == 0 })
  case zeros {
    True -> sum
    False -> {
      let new_list =
        list
        |> list.window_by_2()
        |> it.from_list()
        |> it.map(fn(pair) {
          let assert #(l, r) = pair
          l - r
        })
        |> it.to_list()
      let assert Ok(last) = list.first(new_list)
      extrapolate(new_list, sum + last)
    }
  }
}

fn input() -> List(List(Int)) {
  imp.read_text("day09")
  |> imp.split_lines()
  |> it.from_list()
  |> it.map(fn(string) {
    let assert #(result, []) =
      str.split(string, on: " ")
      |> it.from_list()
      |> it.map(fn(char) { int.parse(char) })
      |> it.to_list()
      |> result.partition()
    result
  })
  |> it.to_list()
}

fn solve(iter: it.Iterator(List(Int))) -> Result(Int, Nil) {
  iter
  |> it.map(fn(list) {
    let assert Ok(last) = list.first(list)
    task.async(fn() { extrapolate(list, last) })
  })
  |> it.map(task.await_forever)
  |> it.reduce(fn(acc, item) { acc + item })
}

fn solve1(lists: List(List(Int))) -> Result(Int, Nil) {
  lists
  |> it.from_list()
  |> solve()
}

fn solve2(lists: List(List(Int))) -> Result(Int, Nil) {
  lists
  |> it.from_list()
  |> it.map(fn(list) {list.reverse(list)})
  |> solve()
}

pub fn main() {
  let input = input()
  io.debug(solve1(input))
  io.debug(solve2(input))
}
