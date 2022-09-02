(function Counter ()
	(let val 0)
	(funcap inc () (val)
		(set val (+ val 1))
	)
	(funcap get () (val)
		val
	)
	(object inc get)
)

(let
	c1 (Counter)
	c2 (Counter)
)

(c1'inc)
(c1'inc)
(c1'inc)
(c1'inc)
(c2'inc)

(print (c1'get) "\n")
(print (c2'get) "\n")
