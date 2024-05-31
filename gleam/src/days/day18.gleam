import gleam/float
import gleam/int
import gleam/io
import gleam/iterator as it
import gleam/string as str
import util/import_util as imp

type Pos =
  #(Int, Int)

pub type Dir {
  Down(Int, String)
  Up(Int, String)
  Left(Int, String)
  Right(Int, String)
}

fn input() -> List(String) {
  imp.read_text("day18")
  |> imp.split_lines()
}

pub fn parse(from line: String) -> Dir {
  let assert [dir, len, col] = str.split(line, " ")
  let assert Ok(len) = int.parse(len)

  case dir {
    "U" -> Up(len, col)
    "D" -> Down(len, col)
    "R" -> Right(len, col)
    "L" -> Left(len, col)
    _ -> panic as "Unexpected token"
  }
}

fn translate(from dir: Dir) -> Dir {
  let string = case dir {
    Down(_, s) -> s
    Up(_, s) -> s
    Left(_, s) -> s
    Right(_, s) -> s
  }

  let assert Ok(num) =
    str.slice(string, 2, 5)
    |> int.base_parse(16)
  let assert Ok(dir) =
    str.slice(string, 7, 1)
    |> int.base_parse(16)

  case dir {
    0 -> Right(num, "")
    1 -> Down(num, "")
    2 -> Left(num, "")
    3 -> Up(num, "")
    a -> panic as {"cannot translate " <> int.to_string(a)}
  }
}

fn move(pos: Pos, dir: Dir) -> #(Pos, Int) {
  let #(x, y) = pos
  case dir {
    Down(len, _) -> #(#(x, y + len), len)
    Up(len, _) -> #(#(x, y - len), len)
    Left(len, _) -> #(#(x - len, y), len)
    Right(len, _) -> #(#(x + len, y), len)
  }
}

fn dig(
  dirs: List(Dir),
  pos: Pos,
  edges: List(Pos),
  area: Int,
) -> #(List(Pos), Int) {
  case dirs {
    [] -> #(edges, area)
    [dir, ..rest] -> {
      let #(pos, len) = move(pos, dir)
      dig(rest, pos, [pos, ..edges], area + len)
    }
  }
}

fn area(edges: List(Pos), sum: Int) -> Float {
  case edges {
    [_] -> {
      let assert Ok(res) = float.divide(int.to_float(sum), 2.0)
      float.absolute_value(res)
    }
    [a, b, ..rest] -> {
      let #(x1, y1) = a
      let #(x2, y2) = b
      let frag = {
        { x1 * y2 } - { y1 * x2 }
      }
      area([b, ..rest], sum + frag)
    }
    _ -> panic as "cannot calculate area"
  }
}

pub fn solve1(dirs: List(Dir)) -> Float {
  let #(edges, edges_area) = dig(dirs, #(0, 0), [#(0, 0)], 0)
  let assert Ok(edges_area) =
    int.to_float(edges_area)
    |> float.divide(2.0)
  area(edges, 0)
  |> float.add(edges_area)
  |> float.add(1.0)
}

pub fn solve2(dirs: List(Dir)) -> Float {
  let #(edges, edges_area) =
    it.from_list(dirs)
    |> it.map(translate)
    |> it.to_list()
    |> dig(#(0, 0), [#(0, 0)], 0)

  let assert Ok(edges_area) =
    int.to_float(edges_area)
    |> float.divide(2.0)
  area(edges, 0)
  |> float.add(edges_area)
  |> float.add(1.0)
}

pub fn main() {
  let input =
    input()
    |> it.from_list()
    |> it.map(parse)
    |> it.to_list()
  io.debug(solve2(input))
}
