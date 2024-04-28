import gleam/list
import gleam/string
import simplifile as file
import gleam/io

fn input_path() -> String {
  let r = file.current_directory()
  case r {
    Ok(x) -> {
      x <> "/../input/"
    }
    Error(x) -> {
      io.print(file.describe_error(x))
      panic
    }
  }
}

pub fn read_text(filename: String) -> String {
  let a = {
    { input_path() <> filename <> ".in" }
    |> file.read
  }
  case a {
    Ok(x) -> x
    Error(x) -> {
      io.println(file.describe_error(x))
      panic
    }
  }
}

pub fn split_lines(input: String) -> List(String) {
  input
  |> string.split(on: "\n")
  |> list.map(with: string.trim)
}
