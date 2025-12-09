# if only i knew how to work with chars
(defn ch [s] (in s 0))

(defn read-file-as-lines [fpath]
  (string/split "\n" (file/read (file/open fpath) :all)))

(defn part1 [fpath]
  (def lines-raw (read-file-as-lines fpath))
  (def lines (map (fn (line) (buffer/slice line)) lines-raw))

  (var split-counter 0) 

  (for l 1 (length lines)
    (def prev-line (in lines (- l 1)))
    (def line (in lines l))

    (for col 0 (length line)
      # if only i knew if it's possible to do enums
      (def above (case (in prev-line col)
        (ch "S") "beam"
        (ch "|") "beam"
        (ch ".") "dot"
        (ch "^") "split"
        (error "invalid prev line char")))

      (def this (case (in line col)
        (ch ".") "dot"
        (ch "^") "split"
        (ch "|") "beam"
        (error "invalid current line char")))

      (match [above this]
        ["beam" "dot"] (do (print "S . :: " above " " this " -> beam falls")
          (put line col (ch "|")))
        ["beam" "split"] (do (print "S . :: " above " " this " -> beam splits")
          (put line (- col 1) (ch "|"))
          (put line (+ col 1) (ch "|"))

          (set split-counter (+ split-counter 1)))
        _ nil)))

  
  # (printf "lines: %q" lines)
  (map (fn (x) (printf "%q" x)) lines)
  (print "split: " split-counter))

(do
  (part1 "test.txt")
  (part1 "input.txt"))

# NOTE: thoughts for part2:
# for part2 we can keep track of how many "timelines" overlap in a certain point.
# that way we can simply get the sum of the row and add it to the total counter
#
#    1    (timelines: 1)
#    ^    (splits: 1)
#   1.1   (timelines: 2)
#   ^ ^   (splits: 3)
#  1 2 1  (timelines: 4)
#  ^ ^ ^  (splits: 6)
# 1 3 3 1 (timelines: 8)
#     ^   (splits: 7)
# 1 33 31 (timelines: 11)
