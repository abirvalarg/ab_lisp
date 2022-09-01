(function Vec (x-pos y-pos)
	(funcap x () (x-pos)
		x-pos
	)
	(funcap y () (y-pos)
		y-pos
	)
	(funcap len () (x-pos y-pos)
		(sqrt (+ (* x-pos x-pos) (* y-pos y-pos)))
	)
	(object x y len)
)

(let a (Vec 4 10))
(put-str "Vector {" (a'x) "; " (a'y) "} has length " (a'len) "\n")
