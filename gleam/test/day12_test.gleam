import util/import_util as imp
import gleeunit
import gleeunit/should
import days/day12
import gleam/iterator as it

fn input() {
  imp.read_text("day12_test")
  |> imp.split_lines()
  |> parse()
}

fn parse(ls: List(String)) {
  ls
  |> it.from_list()
  |> it.map(day12.parse_line)
  |> it.to_list()
}

pub fn example_data_test() {
  input()
  |> day12.solve1()
  |> should.equal(21)
}

pub fn edge_case_test() {
  ["???#??.??????.??#.. 4,3", ".##.?#??.#.?# 2,1,1,1", "###.### 3"]
  |> parse()
  |> day12.solve1()
  |> should.equal(4)
}

pub fn edge_case2_test() {
  let #(input, req) = day12.parse_line("#??##???????????. 1,11,1")
  day12.handle(input, req)
  |> should.equal(3)
}

pub fn doubles_test() {
  [
    "?#?###???#??#?.??? 11,1,1", ".????#?#???#??????? 12,1,1",
    "?#???##??#?????#.??? 2,11,1", "#??##???????????. 1,11,1",
    ".??.?.?#?##?#???#?? 1,11", "??#.?#???####??#??.? 1,11,1",
  ]
  |> parse()
  |> it.from_list()
  |> it.map(fn(item) {
    let #(input, req) = item
    day12.handle(input, req)
  })
  |> it.to_list()
  |> should.equal([3, 10, 7, 3, 6, 2])
}

pub fn main() {
  gleeunit.main()
}
