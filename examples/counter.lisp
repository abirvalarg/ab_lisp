(function create ()
	(let val 0)
	(funcap res () (val)
		(let res val)
		(set val (+ val 1))
		res
	)
)

(let c (create))
(put-str (c) "\n")
(put-str (c) "\n")
(put-str (c) "\n")
