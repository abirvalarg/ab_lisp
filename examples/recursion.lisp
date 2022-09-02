(function rec (stop)
	(if stop
		(print "stopping recursion\n")
		(do
			(print "recursion!!!\n")
			(rec 1)
		)
	)
)
(rec)
