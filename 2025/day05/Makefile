.PHONY: build
build:
	mkdir -p out
	koka --include=libs/koka-std --outputdir=out main.kk

.PHONY: run
run: build
	out/main__main
