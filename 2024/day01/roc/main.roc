app [main] {
    pf: platform "https://github.com/roc-lang/basic-cli/releases/download/0.17.0/lZFLstMUCUvd5bjnnpYromZJXkQUrdhbva4xdBInicE.tar.br",
}

import pf.Stdout
import pf.Path exposing [Path]
import pf.Arg

readFileToStr : Path -> Task Str [ReadFileErr Str]_
readFileToStr = \path ->
    path
    |> Path.readUtf8

# getFirstArg : List Str -> Task Str [Exit (Num *) Str]_
getFirstArg = \args ->
    argResult = List.get args 1 |> Result.mapErr (\_ -> ZeroArgsGiven)

    when argResult is
        Ok arg ->
            Task.ok arg

        Err ZeroArgsGiven ->
            Task.err (Exit 1 "Error ZeroArgsGiven:\n\tI expected one argument, but I got none.\n\tRun the app like this: `roc main.roc -- path/to/input.txt`")

# AppendIfOk : List a, Result * * -> List a
# appendIfOk = \list, result ->
#     when result is
#         Ok x ->
#             List.append list x
#         Err e ->
#             list

# ListKeepOks : List (Result * *) -> List *
listKeepOks = \lst ->
    # z : List *
    z =
      List.walk lst [] (\acc, item -> when item is
          Ok x ->
              dbg(x)
              List.append acc x
          Err _ ->
              acc
      )

    # dbg(z)

    z

# listGetUnchecked = \list, index ->
#     List.get list index
#     |> Result.

# xAppend : List List Str
xAppend : List List Str, List Str -> Result (List List Str) [OutOfBounds]
xAppend = \listofLists, listofItems ->
    list0 : List Str
    list0 = List.get? listofLists 0
    list1 : List Str
    list1 = List.get? listofLists 1

    item0 : Str
    item0 = List.get? listofItems 0
    item1 : Str
    item1 = List.get? listofItems 1

    # list0 ++ [item0], list1 ++ [item1]
    # List.append list0 item0, List.append list1 item1
    Ok [List.append list0 item0, List.append list1 item1]

run : Str -> Result (List List Str) [OutOfBounds]
run = \fileContents ->
    listofLists : List List Str
    listofLists =
        fileContents
        |> Str.splitOn "\n"
        |> List.map (\line -> line |> Str.splitOn "   ")

    # first : List Str
    maybefirst = List.get listofLists 0
    first = when maybefirst is
        Ok x ->
            x
        Err e ->
            return Err e

    # Stdout.line! "$(first)"
    firstfirst : Result Str [OutOfBounds]
    firstfirst = List.get (first) 0

    ff = when firstfirst is
        Ok s ->
            s
        Err e ->
            return Err e

    # ff : Str
    # ff = firstfirst?

    y : Result (List (List Str)) [OutOfBounds]
    y = List.walkTry listofLists [[], []] xAppend

    z : List (List Str)
    z = y?

    arr1 : List Str
    arr1 = List.get? z 0
    # arr12 = List.sortAsc arr1
    # arr12 : List (Result U64 _)
    # arr12 : List U64
    # arr12 = List.map arr1 (\x -> Ok Num.toU64Checked? x)
    arr123 = List.keepOks arr1 (\x -> Ok (Str.toI64? x))
    # arr123 : List U64
    # arr123 = listKeepOks arr12
    # arr1234 = List.sortAsc arr123

    # arr2 = List.sortAsc (List.get? z 1)

    Ok [arr123]
    # Ok [["hello"]]

listOfListsToStr : List List Str -> Str
listOfListsToStr = \listOfLists ->
    listOfLists
        |> List.map (\list -> list |> Str.joinWith " + ")
        |> Str.joinWith " | "


main =
    args = Arg.list! {}
    firstArg = getFirstArg! (args)

    fileContentStr = Str.trim (readFileToStr! (Path.fromStr firstArg))

    Stdout.line! "$(fileContentStr)"

    # listofLists =
    #     fileContentStr
    #     |> Str.splitOn "\n"
    #     |> List.map (\line -> line |> Str.splitOn "   ")

    # # Stdout.line! "$(y)"

    # # y = List.get? listofLists 0
    # # y =
    # #     listofLists
    # #     |> List.map (\list -> list |> Str.joinWith " a ")
    # #     |> Str.joinWith " b "
    # # y = List.walk listofLists [[], []] xAppend

    # # z =
    # #     y
    # #     |> List.map (\list -> list |> Str.joinWith " a ")
    # #     |> Str.joinWith " b "

    # first = List.get listofLists 0

    # Stdout.line! "$(first)"

    expect Str.trimEnd "hello\n" == "hello"

    # result = run "1   2\n3   4\n5   6\n"
    result = run fileContentStr
    when result is
        Ok x ->
            Stdout.line! "woot: $(listOfListsToStr x)"

        Err e ->
            when e is
                OutOfBounds ->
                    Task.err (Exit 1 "OutOfBounds")
