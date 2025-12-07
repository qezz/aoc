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

// source: rosetta code + mod for cmp
fun qsort( xs : list<a>, cmp : ( a : a, b : a) -> bool) : div list<a> {
  match(xs) {
    Cons(x,xx) -> {
      val (ys, zs) = xx.partition fn(el) { cmp(el, x) }
      qsort(ys, cmp) ++ [x] ++ qsort(zs, cmp)
    }
    Nil -> Nil
  }
}

pub fun merge-ranges-unsafe(a : idrange, b : idrange) : idrange {
  Idrange(min(a.from, b.from), max(a.to, b.to))
}

pub fun x-overlaps-with-item(item : idrange, x: idrange) : bool {
  if item.to >= (x.from - 1) then True else False
}

pub fun add-range-but-its-safe-only-if-ranges-are-sorted(ranges : list<idrange> , x : idrange) : list<idrange> {
  match ranges
    Nil -> Cons(x, Nil)
    Cons(head, tail) ->
      if x-overlaps-with-item(head, x) then
        Cons(merge-ranges-unsafe(head, x), tail)
      else
        Cons(x, ranges)
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

  val part1 = count-in-ranges(reg, ids)
  println("part1: " ++ show(part1))

  println("sorting")
  val sorted = qsort(reg, fn (a, b) a.from < b.from)
  println("sorted: " ++ joinsep/join(sorted.map(idrange-to-string), ", "))

  println("folding")
  val folded-reversed = sorted.foldl(Nil, add-range-but-its-safe-only-if-ranges-are-sorted)
  println("folded: " ++ joinsep/join(folded-reversed.map(idrange-to-string), ", "))

  val sum = folded-reversed.foldl(0, fn (acc, x) acc + x.idrange-length)
  println("part2: " ++ show(sum))
