import gleam/otp/task
import gleam/io
import gleam/iterator as it

type Race {
  Race(time: Int, dist: Int)
}

fn perf(race: Race, offset: Int) -> Int {
  offset * { race.time - offset }
}

fn simulate(race: Race) -> Int {
  let limit = 1000
  let chunk = race.time / limit
  it.range(from: 0, to: race.time)
  |> it.sized_chunk(into: chunk)
  |> it.map(fn(rng) {
    task.async(fn() {
      it.from_list(rng)
      |> it.map(fn(item) { perf(race, item) })
      |> it.filter(fn(res) { res > race.dist })
      |> it.length
    })
  })
  |> it.map(task.await_forever)
  |> it.fold(0, fn(sum, el) {sum + el})
}

fn input() -> List(Race) {
  [Race(46, 214), Race(80, 1177), Race(78, 1402), Race(66, 1024)]
}

fn solve1(races: List(Race)) -> Int {
  races
  |> it.from_list
  |> it.map(simulate)
  |> it.fold(1, fn(acc, el) {acc * el})
}


fn solve2() -> Int {
  Race(46807866, 214117714021024)
  |> simulate()
}

pub fn main() {
  let races = input()
  io.debug(solve1(races))
  io.debug(solve2())
}
