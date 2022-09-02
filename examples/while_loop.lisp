(function print-list (list)
	(while list (do
		(print (head list) "\n")
		(set list (tail list))
	))
)
(let a '('a 'b 'c 'd))
(print-list a)
(print a "\n")
