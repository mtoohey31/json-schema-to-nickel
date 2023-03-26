let any_of: forall a. Array (a -> Bool) -> a -> Bool =
  fun predicates value => array.foldl (fun current predicate => current || predicate value) false predicates
in
null | contract.from_predicate (any_of [builtin.is_num, builtin.is_bool])
