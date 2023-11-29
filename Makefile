ENDPOINT ?= arb-one.streamingfast.io:443 
#START_BLOCK ?= 12292922
#142,415,715
#149808983 
#149,811,551
START_BLOCK ?= 149811551 
#STOP_BLOCK ?= +2000
#START_BLOCK ?= 150414818
#meSTART_BLOCK ?= 154415638
#START_BLOCK ?= 154260315
#START_BLOCK ?= 154268963
STOP_BLOCK ?= +100000
#STOP_BLOCK ?= +10000 

.PHONY: build
build:
	cargo build --target wasm32-unknown-unknown --release

.PHONY: run
run: build
	substreams run -e $(ENDPOINT) substreams.yaml map_short_decreases -s $(START_BLOCK) -t $(STOP_BLOCK)

.PHONY: run2 
run2: build
	substreams run -e $(ENDPOINT) substreams.yaml map_increases -s $(START_BLOCK) -t $(STOP_BLOCK)

.PHONY: gui
gui: build
	substreams gui -e $(ENDPOINT) substreams.yaml map_increases -s $(START_BLOCK) -t $(STOP_BLOCK)

.PHONY: protogen
protogen:
	substreams protogen ./substreams.yaml --exclude-paths="google,sf/substreams/rpc,sf/substreams/v1"

.PHONY: pack
pack: build
	substreams pack substreams.yaml
