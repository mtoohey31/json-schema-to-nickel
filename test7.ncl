let AnyOf =
  fun contract_producers label value =>
    let maybe_contract = array.foldl (fun maybe_partial_contract contract_producer =>
        if !(builtin.is_fun maybe_partial_contract) then null
        else contract_producer value) (contract.from_predicate false) contract_producers in
    if builtin.is_fun maybe_contract then maybe_contract null
    else value
in
let NeverWithMsg = fun msg label _value =>
  contract.blame_with msg label in
let greater_than' = fun exmin value => if value <= exmin
  then value | (NeverWithMsg "less than foo")
  else null in
3 | AnyOf [ (greater_than' 5) ]
