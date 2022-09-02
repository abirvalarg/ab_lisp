(let
	a '('a 'b 'c)
	b (cons 'first 'second a)
)
(print
	(head b) "\n"
	(tail b) "\n"
	a "\n"
	b "\n"
)
