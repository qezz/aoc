import std/os/file
import std/os/path
import std/core/string
import std/core
import std/core/list

struct idrange
  from : int
  to : int

pub fun idrange-to-string(idrange) : string {
  "[" ++ idrange.from.show ++ ", " ++ idrange.to.show  ++ "]"
}

pub fun in-idrange(idrange : idrange, id: int) : bool {
  id >= idrange.from && id <= idrange.to
}

pub fun idrange-from-row(row) : exn idrange {
  val parts = sep/split(row, "-")
  val from = int/parse-int(parts[0].unjust).unjust
  val to = int/parse-int(parts[1].unjust).unjust

  Idrange(from, to)
}

pub fun parse-idranges(a : list<string>) : exn  list<idrange> {
  a.map(idrange-from-row)
}

pub fun parse-ids(a : list<string>) : exn list<int> {
  a.map(fn (x) int/parse-int(x).unjust)
}

pub fun id-in-ranges(ranges: list<idrange>, id: int) : bool {
  ranges.any(fn(range) in-idrange(range, id))
}

pub fun count-in-ranges(idranges : list<idrange>, ids: list<int>) : int {
  ids.filter(fn (id) id-in-ranges(idranges, id)).length
}

fun main()
  val inputpath = path("input.txt")
  val input = read-text-file(inputpath)

  // parse-lines(sep/split(input, "\n\n"))
  val input-parts = sep/split(input, "\n\n")
  val raw-reg = input-parts[0].unjust
  val raw-things = trim(input-parts[1].unjust)

  val reg = parse-idranges(sep/split(raw-reg, "\n"))
  val ids = parse-ids(sep/split(raw-things, "\n"))

  // concat/join(reg.map(idrange-to-string))
  // joinsep/join(ids.map(show), ",")

  count-in-ranges(reg, ids)
