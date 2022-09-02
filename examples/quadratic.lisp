; ax² + bx + c = 0
(function solve (a b c)
	(let d (- (* b b) (* 4 a c)) )
	(if (>= d)
		(do
			(let sqrt-d (sqrt d) )
			'(
				(/ (- (- b) sqrt-d) (* 2 a))
				(/ (+ (- b) sqrt-d) (* 2 a))
			)
		)
		()
	)
)

(debug
	(solve 23 1 45)
	(solve 1 -35 -450)
)
