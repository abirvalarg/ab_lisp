(function rec (stop)
	(if stop
		(put-str "stopping recursion\n")
		(do
			(put-str "recursion!!!\n")
			(rec 1)
		)
	)
)
(rec)
