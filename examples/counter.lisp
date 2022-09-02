(function create ()
	(let val 0)
	(funcap res () (val)
		(let res val)
		(set val (+ val 1))
		res
	)
)

(let
	c1 (create)
	c2 (create)
)
(print (c1) "\n")
(print (c2) "\n")
(print (c1) "\n")
(print (c2) "\n")
(print (c1) "\n")
(print (c2) "\n")
