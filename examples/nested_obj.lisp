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

(function Object (pos)
	(object pos)
)

(let a (Object (Vec 3 10)))
(print (a'pos'x) "\n")
