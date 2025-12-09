# if only i knew how to work with chars
(defn ch [s] (in s 0))

(defn read-file-as-lines [fpath]
  (filter (fn (x) (> (length x) 0)) (string/split "\n" (file/read (file/open fpath) :all))))

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

(defn char-str-to-pair [char-str]
  (case char-str
    (ch "S") (do (print "s") [:beam 1])
    (ch ".") [:beam 0] # assumption: space is technically a beam 0 times.
    (ch "^") [:split]
    (error "fail")))

(defn part2[fpath]
  (def lines-raw (read-file-as-lines fpath))
  (def lines-chars (map (fn (line) (buffer/slice line)) lines-raw))
  (map (fn (x) (printf "%q" x)) lines-chars)
  (def lines (map (fn (line) (map char-str-to-pair line)) lines-chars))
  # (map (fn (x) (printf "%q" x)) lines)

  (var split-counter 0)

  (for l 1 (length lines)
    (def prev-line (in lines (- l 1)))
    (def line (in lines l))

    (for col 0 (length line)
      (def above (in prev-line col))
      (def above-right (get prev-line (+ col 1)))

      (def this (in line col))
      (def right (get line (+ col 1)))

      (match [right above-right]
        [[:beam 0] [:beam n]]
        (if (> n 0)
          # put it to right, then continue
          (put line (+ col 1) above-right)
        )
        )

      (match [above this]
        [[:beam n] [:beam m]]
          (do
            (if (and (> n 0) (= m 0))
              (do
                (put line col [:beam (+ m n)])
                )

              nil))
        [[:beam n] [:split]]
          (do
            (if (> n 0)
              (do
                # the right could have been modifier above, so we need to re-read it
                (def left (in line (- col 1)))
                (def right (in line (+ col 1)))

                (match [left right]
                  [[:beam l] [:beam r]] (do
                    (put line (- col 1) [:beam (+ l n)])
                    (put line (+ col 1) [:beam (+ r n)]))
                  (error "unreach")
                  )

                )
              nil)))))

  # (print "\n\nafter:")
  # (map (fn (x) (printf "%q" x)) lines)

  (printf "%q" (reduce (fn (acc beam) (+ acc (in beam 1))) 0 (last lines))))

(do
  (part1 "test.txt")
  (part1 "input.txt")

  (part2 "test.txt")
  (part2 "input.txt")
  )

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
