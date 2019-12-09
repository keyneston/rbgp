build_bird:
	docker build -t bird scripts/bird/

run_bird:
	docker run -p 2179:179 -t bird  

shell_bird:
	docker run -ti bird /bin/bash