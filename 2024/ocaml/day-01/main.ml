let input = "3   4\n4   3\n2   5\n1   3\n3   9\n3   3"

let sum_pairs (first : 'a list) (second : 'a list) =
  let rec sum (one : 'a list) (two : 'a list) (acc : int) =
    match (one, two) with
    | [], [] -> acc
    | x :: first_rest, y :: second_rest ->
        sum first_rest second_rest (max x y - min x y + acc)
    | _ -> failwith "test"
  in
  sum first second 0

let split_pairs pairs =
  let rec aux pairs acc1 acc2 =
    match pairs with
    | [] -> (acc1, acc2)
    | (x, y) :: rest -> aux rest (x :: acc1) (y :: acc2)
  in
  aux pairs [] []

let parse_input (input : string) : int =
  let (lines : (int * int) list) =
    input |> String.split_on_char '\n'
    |> List.map (fun line ->
           line |> String.split_on_char ' '
           |> List.filter (fun c -> compare c "" != 0)
           |> List.map int_of_string
           |> fun values ->
           match values with
           | [ first; second ] -> (first, second)
           | _ -> failwith "invalid input")
  in

  let first, second =
    split_pairs lines |> fun (first, second) ->
    (List.sort compare first, List.sort compare second)
  in
  sum_pairs first second

let () =
  let result = parse_input input in
  print_endline (string_of_int result)
