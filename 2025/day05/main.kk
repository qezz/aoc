module main

import std/os/file
import std/os/path
import std/core/string
import std/core
import std/core/list
import std/data/hashset
import std/data/hash

struct idrange
  from : int
  to : int

pub fun idrange-to-string(idrange) : string {
  "[" ++ idrange.from.show ++ ", " ++ idrange.to.show  ++ "]"
}

pub fun idrange-length(idrange) : int {
  idrange.to - idrange.from + 1
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

// pub fun rec-insert( hs : hash-set<v>, l : list<v>): hash-set<v> {
fun from-list-helper( hs : hash-set<v>, l : list<v>, ?hash : (v, int64) -> int, ?(==) : (v, v) -> bool ) : hash-set<v> {
  match l
    Nil -> hs
    Cons(x, xs) -> 
      from-list-helper( hs.insert( x ), xs )
}

pub fun add-idrange-to-hashset(hs: hash-set<int>, idrange: idrange) : console hash-set<int> {
  print(".")
  val r = range/list(idrange.from, idrange.to)

  // r.foldl(hs, fn (acc, x) acc.insert(x))
  from-list-helper(hs, r)
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

// pub fun add-to-hm(hm: hashmap<int>, x: int) : hashmap<int> {
//   hm.insert(x)
// }

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

  val part1 = count-in-ranges(reg, ids)
  println("part1: " ++ show(part1))

  // val tr = Idrange(10, 15);
  // val rang = range/list(tr.from, tr.to + 1)
  // println("range: " ++ joinsep/join(rang.map(show), ","))
  // val r = rang.foldl(hash-set(), fn (acc, x) acc.insert(x))
  // r.map(fn (v) println(v))
  // val xxx = std/data/hashset/effect/map(r, fn (v) println("hm val: " ++ show(v)))
  // println(xxx)

  // println("> " ++ Idrange(10, 15))

  println("folding")
  val part2 = reg.foldl(hash-set(), fn (acc, lst) add-idrange-to-hashset(acc, lst))
  println("folded")

  // val l = std/data/hashset/effect/map(r, fn (v) show(v))

  // val sum = std/data/hashset/effect/map
  val count = 0
  // val vec-of-lists = part2.data.map(fn (buckets) buckets.list/map(fn (x) x) )
  val vec-of-lists = part2.data
  // val list-of-lists: list<list<_a>> = std/core/vector/map(vec-of-lists, fn (x) x.vector/list())
  val list-of-lists = vec-of-lists.list()
  val res = list-of-lists.flatmap(fn (x) x)
  val amount = res.length()
  println(amount)
  // println("part2: " ++ part2)
  println("done")
