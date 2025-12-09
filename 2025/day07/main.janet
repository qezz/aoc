# if only i knew how to work with chars
(defn ch [s] (in s 0))

(defn read-file-as-lines [fpath]
  (def f (file/read (file/open fpath) :all))

  (->> f
       (string/split "\n")
       (filter (fn (x) (> (length x) 0)))))

(defn part1 [name fpath]
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
        ["beam" "dot"]
          (put line col (ch "|"))
        ["beam" "split"] (do
          (put line (- col 1) (ch "|"))
          (put line (+ col 1) (ch "|"))

          (set split-counter (+ split-counter 1)))
        _ nil)))

  (printf "%s: part1: split: %d" name split-counter))

(defn char-str-to-pair [char-str]
  (case char-str
    (ch "S") [:beam 1]
    (ch ".") [:beam 0] # assumption: space is technically a beam 0 times.
    (ch "^") [:split]
    (error "fail")))

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
(defn part2 [name fpath]
  (def lines (->> fpath
                  (read-file-as-lines)
                  (map buffer/slice)
                  (map (fn (line) (map char-str-to-pair line)))))

  (for l 1 (length lines)
    (def prev-line (in lines (- l 1)))
    (def line (in lines l))

    (for col 0 (length line)
      (def above (in prev-line col))
      (def above-right (get prev-line (+ col 1)))

      (def this (in line col))
      (def right (get line (+ col 1)))

      # assumption: it's enough to perform this once to the right
      # cell because we are iterating left to right, so the data
      # should be complete by the time we process the next cell.
      (match [right above-right]
        [[:beam 0] [:beam n]]
        (if (> n 0)
          # put it to right, then continue
          (put line (+ col 1) above-right)))

      (match [above this]
        [[:beam n] [:beam m]]
          (if (and (> n 0) (= m 0))
            (put line col [:beam (+ m n)]))
        [[:beam n] [:split]]
          (if (> n 0)
            (do
              # the right could have been modifier above, so we need to re-read it
              (def left (in line (- col 1)))
              (def right (in line (+ col 1)))

              (match [left right]
                [[:beam l] [:beam r]]
                  (do
                    (put line (- col 1) [:beam (+ l n)])
                    (put line (+ col 1) [:beam (+ r n)]))
                (error "unreachable")))))))

  (printf "%s: part2: timelines: %q"
          name
          (->> lines
               (last)
               (reduce (fn (acc beam) (+ acc (in beam 1))) 0))))

(do
  (part1 "test" "test.txt")
  (part2 "main" "test.txt")

  (part1 "test" "input.txt")
  (part2 "main" "input.txt"))
