let Type = contract.from_predicate (fun v => match { `Num => true, _ => false} (builtin.typeof v)) in
false | Type
