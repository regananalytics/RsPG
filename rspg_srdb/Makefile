REPO=https://github.com/BTMorton/dnd-5e-srd.git
OUTDIR=srd-data

.PHONY: all

clean:
ifneq ($(wildcard ./${OUTDIR}/.),)
	rm -rf ${OUTDIR}
endif

setup:
	git clone ${REPO} ${OUTDIR}

deploy: # alias for up
	make up

up:
ifneq ($(wildcard ./${OUTDIR}/.),)
	docker-compose up -d
else
	make setup
	make up
endif

down:
	docker-compose down