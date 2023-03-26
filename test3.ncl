let A = { a | Num, .. } in
let B = { b | Num, .. } in
let AllOf = fun contracts label value =>
    array.foldl (fun value con => contract.apply con label value) value contracts in
let If = fun predicate con label value =>
    if predicate value then contract.apply con label value else value in
# { a = null, b = 3, f = 6 } | AllOf [ A, B ]
[{ c = false }] | If builtin.is_record { c | Num, .. }
