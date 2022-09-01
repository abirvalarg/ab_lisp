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
(put-str (c1) "\n")
(put-str (c2) "\n")
(put-str (c1) "\n")
(put-str (c2) "\n")
(put-str (c1) "\n")
(put-str (c2) "\n")
