cgranges_version = 0.1.1
iitii_version = bc049182fc6aecf5b48316bca9bd57785673782d
clinvar_version = 20240730

domain = $(shell seq 1 2 14 | awk '{print 2**$$1}')

iitiiri_build_name = $(shell echo $(domain) | awk -v RS=' ' '{print "iitiiri_build_"$$1}')
iitiiri_query_name = $(shell echo $(domain) | awk -v RS=' ' '{print "iitiiri_query_"$$1}' )
iitiiri_annotation_variant_name = $(shell echo $(domain) | awk -v RS=' ' '{print "iitiiri_annotation_variant_"$$1}' )
iitiiri_build_p_name = $(shell echo $(domain) | awk -v RS=' ' '{print "iitiiri_parallel_build_"$$1}')
iitiiri_query_p_name = $(shell echo $(domain) | awk -v RS=' ' '{print "iitiiri_parallel_query_"$$1}' )
iitiiri_annotation_variant_p_name = $(shell echo $(domain) | awk -v RS=' ' '{print "iitiiri_parallel_annotation_variant_"$$1}' )

iitiiri_build_path = $(shell echo $(iitiiri_build_name) | awk -v RS=' ' '{print "./bin/"$$1}')
iitiiri_query_path = $(shell echo $(iitiiri_query_name) | awk -v RS=' ' '{print "./bin/"$$1}')
iitiiri_annotation_variant_path = $(shell echo $(iitiiri_annotation_variant_name) | awk -v RS=' ' '{print "./bin/"$$1}')
iitiiri_build_p_path = $(shell echo $(iitiiri_build_p_name) | awk -v RS=' ' '{print "./bin/"$$1}')
iitiiri_query_p_path = $(shell echo $(iitiiri_query_p_name) | awk -v RS=' ' '{print "./bin/"$$1}')
iitiiri_annotation_variant_p_path = $(shell echo $(iitiiri_annotation_variant_p_name) | awk -v RS=' ' '{print "./bin/"$$1}')

iitii_hyper_name = $(shell echo $(domain) | awk -v RS=' ' '{print "-n iitii_"$$1}')
iitiiri_hyper_name = $(shell echo $(domain) | awk -v RS=' ' '{print "-n iitiiri_"$$1}')
iitiiri_hyper_p_name = $(shell echo $(domain) | awk -v RS=' ' '{print "-n iitiiri_parallel_"$$1}')

iitii_build_hyper_cmd = $(shell echo $(domain) | awk -v RS=' ' '{print "\"./bin/iitii_build dataset/$*.bed "$$1"\""}')
iitii_annotation_variant_hyper_cmd = $(shell echo $(domain) | awk -v RS=' ' '{print "\"./bin/iitii_annotation_variant dataset/hg38.bed dataset/$*.bed "$$1"\""}')
iitiiri_build_hyper_cmd = $(shell echo $(iitiiri_build_path) | awk -v RS=' ' '{print "\""$$1" dataset/$*.bed""\""}')
iitiiri_build_hyper_p_cmd = $(shell echo $(iitiiri_build_p_path) | awk -v RS=' ' '{print "\""$$1" dataset/$*.bed""\""}')
iitiiri_annotation_variant_hyper_cmd = $(shell echo $(iitiiri_build_path) | awk -v RS=' ' '{print "\""$$1" dataset/hg38.bed dataset/$*.bed\""}')
iitiiri_annotation_variant_hyper_p_cmd = $(shell echo $(iitiiri_build_p_path) | awk -v RS=' ' '{print "\""$$1" dataset/hg38.bed dataset/$*.bed\""}')

thread_effect_hyper_name_0 = $(shell echo $(domain) | awk -v RS=' ' '{print "-n parallel_"$$1"_0"}')
thread_effect_hyper_cmd_0 = $(shell echo $(domain) | awk -v RS=' ' '{print "\"RAYON_NUM_THREADS=1 ./bin/iitiiri_build_"$$1" dataset/$*.bed\""}')
thread_effect_hyper_name_1 = $(shell echo $(domain) | awk -v RS=' ' '{print "-n parallel_"$$1"_1"}')
thread_effect_hyper_cmd_1 = $(shell echo $(domain) | awk -v RS=' ' '{print "\"RAYON_NUM_THREADS=1 ./bin/iitiiri_parallel_build_"$$1" dataset/$*.bed\""}')
thread_effect_hyper_name_2 = $(shell echo $(domain) | awk -v RS=' ' '{print "-n parallel_"$$1"_2"}')
thread_effect_hyper_cmd_2 = $(shell echo $(domain) | awk -v RS=' ' '{print "\"RAYON_NUM_THREADS=2 ./bin/iitiiri_parallel_build_"$$1" dataset/$*.bed\""}')
thread_effect_hyper_name_3 = $(shell echo $(domain) | awk -v RS=' ' '{print "-n parallel_"$$1"_3"}')
thread_effect_hyper_cmd_3 = $(shell echo $(domain) | awk -v RS=' ' '{print "\"RAYON_NUM_THREADS=3 ./bin/iitiiri_parallel_build_"$$1" dataset/$*.bed\""}')
thread_effect_hyper_name_4 = $(shell echo $(domain) | awk -v RS=' ' '{print "-n parallel_"$$1"_4"}')
thread_effect_hyper_cmd_4 = $(shell echo $(domain) | awk -v RS=' ' '{print "\"RAYON_NUM_THREADS=4 ./bin/iitiiri_parallel_build_"$$1" dataset/$*.bed\""}')
thread_effect_hyper_name_5 = $(shell echo $(domain) | awk -v RS=' ' '{print "-n parallel_"$$1"_5"}')
thread_effect_hyper_cmd_5 = $(shell echo $(domain) | awk -v RS=' ' '{print "\"RAYON_NUM_THREADS=5 ./bin/iitiiri_parallel_build_"$$1" dataset/$*.bed\""}')
thread_effect_hyper_name_6 = $(shell echo $(domain) | awk -v RS=' ' '{print "-n parallel_"$$1"_6"}')
thread_effect_hyper_cmd_6 = $(shell echo $(domain) | awk -v RS=' ' '{print "\"RAYON_NUM_THREADS=6 ./bin/iitiiri_parallel_build_"$$1" dataset/$*.bed\""}')
thread_effect_hyper_name_7 = $(shell echo $(domain) | awk -v RS=' ' '{print "-n parallel_"$$1"_7"}')
thread_effect_hyper_cmd_7 = $(shell echo $(domain) | awk -v RS=' ' '{print "\"RAYON_NUM_THREADS=7 ./bin/iitiiri_parallel_build_"$$1" dataset/$*.bed\""}')
thread_effect_hyper_name_8 = $(shell echo $(domain) | awk -v RS=' ' '{print "-n parallel_"$$1"_8"}')
thread_effect_hyper_cmd_8 = $(shell echo $(domain) | awk -v RS=' ' '{print "\"RAYON_NUM_THREADS=8 ./bin/iitiiri_parallel_build_"$$1" dataset/$*.bed\""}')


all: benchmark

# Get data
dataset/gnomad.bed:
	@mkdir -p dataset
	@curl https://storage.googleapis.com/gcp-public-data--gnomad/release/2.1.1/vcf/exomes/gnomad.exomes.r2.1.1.sites.2.vcf.bgz | gunzip - | grep -v "^#" | cut -f 1,2,5 |  awk '{print $$1,$$2,$$2+length($$3)}' > dataset/gnomad.bed

dataset/clinvar.bed:
	@mkdir -p dataset
	@curl https://ftp.ncbi.nlm.nih.gov/pub/clinvar/vcf_GRCh38/archive_2.0/2024/clinvar_$(clinvar_version).vcf.gz | gunzip - | grep -v "^#" | cut -f 1,2,5 |  awk '{print $$1,$$2,$$2+length($$3)}' > dataset/clinvar.bed

dataset/hg38.bed:
	@mkdir -p dataset
	@curl https://ftp.ensembl.org/pub/release-92/gff3/homo_sapiens/Homo_sapiens.GRCh38.92.gff3.gz | gunzip - | grep -v "^#" | cut -f 1,4,5 | tr '\t' ' ' > dataset/hg38.bed

dataset/hg001.bed:
	@mkdir -p dataset
	@curl https://ftp-trace.ncbi.nlm.nih.gov/ReferenceSamples/giab/release/NA12878_HG001/latest/GRCh38/HG001_GRCh38_1_22_v4.2.1_benchmark.vcf.gz | gunzip - | grep -v "^#" | cut -f 1,2,5 |  awk '{print $$1,$$2,$$2+length($$3)}' > dataset/hg001.bed

dataset/hg002.bed:
	@mkdir -p dataset
	@curl https://ftp-trace.ncbi.nlm.nih.gov/ReferenceSamples/giab/release/AshkenazimTrio/HG002_NA24385_son/latest/GRCh38/HG002_GRCh38_1_22_v4.2.1_benchmark.vcf.gz | gunzip - | grep -v "^#" | cut -f 1,2,5 |  awk '{print $$1,$$2,$$2+length($$3)}' > dataset/hg002.bed

dataset/hg003.bed:
	@mkdir -p dataset
	@curl https://ftp-trace.ncbi.nlm.nih.gov/ReferenceSamples/giab/release/AshkenazimTrio/HG003_NA24149_father/latest/GRCh38/HG003_GRCh38_1_22_v4.2.1_benchmark.vcf.gz | gunzip - | grep -v "^#" | cut -f 1,2,5 |  awk '{print $$1,$$2,$$2+length($$3)}' > dataset/hg003.bed

dataset/hg004.bed:
	@mkdir -p dataset
	@curl https://ftp-trace.ncbi.nlm.nih.gov/ReferenceSamples/giab/release/AshkenazimTrio/HG004_NA24143_mother/latest/GRCh38/HG004_GRCh38_1_22_v4.2.1_benchmark.vcf.gz | gunzip - | grep -v "^#" | cut -f 1,2,5 |  awk '{print $$1,$$2,$$2+length($$3)}' > dataset/hg004.bed

dataset/hg006.bed:
	@mkdir -p dataset
	@curl https://ftp-trace.ncbi.nlm.nih.gov/ReferenceSamples/giab/release/ChineseTrio/HG006_NA24694_father/latest/GRCh38/HG006_GRCh38_1_22_v4.2.1_benchmark.vcf.gz | gunzip - | grep -v "^#" | cut -f 1,2,5 |  awk '{print $$1,$$2,$$2+length($$3)}' > dataset/hg006.bed

dataset/hg007.bed:
	@mkdir -p dataset
	@curl https://ftp-trace.ncbi.nlm.nih.gov/ReferenceSamples/giab/release/ChineseTrio/HG007_NA24695_mother/latest/GRCh38/HG007_GRCh38_1_22_v4.2.1_benchmark.vcf.gz | gunzip - | grep -v "^#" | cut -f 1,2,5 |  awk '{print $$1,$$2,$$2+length($$3)}' > dataset/hg007.bed

dataset/combine_shuffle.bed: dataset/gnomad.bed dataset/clinvar.bed dataset/hg38.bed dataset/hg001.bed dataset/hg002.bed dataset/hg003.bed  dataset/hg004.bed dataset/hg006.bed dataset/hg007.bed
	@cat dataset/gnomad.bed dataset/clinvar.bed dataset/hg38.bed dataset/hg001.bed \
	dataset/hg002.bed dataset/hg003.bed  dataset/hg004.bed dataset/hg006.bed \
	dataset/hg007.bed | \
	shuf --random-source dataset/hg002.bed > dataset/combine_shuffle.bed


# Build cgranges
cgranges:
	@curl -L https://github.com/lh3/cgranges/archive/refs/tags/v$(cgranges_version).tar.gz | tar xvz
	@mv cgranges-$(cgranges_version) cgranges

bin/cgranges_build: cgranges cgranges_build.c
	@mkdir -p bin
	@cc -O2 -DNDEBUG cgranges/cgranges.c cgranges_build.c -o bin/cgranges_build

bin/cgranges_query: cgranges cgranges_query.c
	@mkdir -p bin
	@cc -O2 -DNDEBUG cgranges/cgranges.c cgranges_query.c -o bin/cgranges_query

bin/cgranges_annotation_variant: cgranges cgranges_annotation_variant.c
	@mkdir -p bin
	@cc -O2 -DNDEBUG cgranges/cgranges.c cgranges_annotation_variant.c -o bin/cgranges_annotation_variant


# Build iitii
iitii:
	@curl -L https://github.com/mlin/iitii/archive/$(iitii_version).tar.gz | tar xvz
	@mv iitii-$(iitii_version) iitii

bin/iitii_build: iitii iitii_build.cpp
	@mkdir -p bin
	@c++ -O2 -DNDEBUG iitii_build.cpp -o bin/iitii_build

bin/iitii_query: bin iitii iitii_query.cpp
	@mkdir -p bin
	@c++ -O2 -DNDEBUG iitii_query.cpp -o bin/iitii_query

bin/iitii_annotation_variant: bin iitii iitii_annotation_variant.cpp
	@mkdir -p bin
	@c++ -O2 -DNDEBUG iitii_annotation_variant.cpp -o bin/iitii_annotation_variant

# Build rust_bio
bin/rust_bio_build: ../../examples/rust_bio_build.rs
	@mkdir -p bin
	@cargo build --release --example rust_bio_build
	@cp ../../target/release/examples/rust_bio_build bin/rust_bio_build

bin/rust_bio_query: ../../examples/rust_bio_query.rs
	@mkdir -p bin
	@cargo build --release --example rust_bio_query
	@cp ../../target/release/examples/rust_bio_query bin/rust_bio_query

# Build iitri
bin/iitri_build: ../../examples/iitri_build.rs
	@mkdir -p bin
	@cargo build --release --example iitri_build
	@cp ../../target/release/examples/iitri_build bin/iitri_build

bin/iitri_parallel_build: ../../examples/iitri_build.rs
	@mkdir -p bin
	@cargo build --release --example iitri_build --features parallel
	@cp ../../target/release/examples/iitri_build bin/iitri_parallel_build

bin/iitri_query: ../../examples/iitri_query.rs
	@mkdir -p bin
	@cargo build --release --example iitri_query
	@cp ../../target/release/examples/iitri_query bin/iitri_query

bin/iitri_parallel_query: ../../examples/iitri_query.rs
	@mkdir -p bin
	@cargo build --release --example iitri_query --features parallel
	@cp ../../target/release/examples/iitri_query bin/iitri_parallel_query


# Build iitiiri
bin/iitiiri_build_%: ../../examples/iitiiri_build.rs
	@mkdir -p bin
	@IITIIRI_DOMAIN=$* cargo build --release --example iitiiri_build
	@cp ../../target/release/examples/iitiiri_build $@

bin/iitiiri_parallel_build_%: ../../examples/iitiiri_build.rs
	@mkdir -p bin
	@IITIIRI_DOMAIN=$* cargo build --release --example iitiiri_build --features parallel
	@cp ../../target/release/examples/iitiiri_build $@

bin/iitiiri_query_%: ../../examples/iitiiri_query.rs
	@mkdir -p bin
	@IITIIRI_DOMAIN=$* cargo build --release --example iitiiri_query
	@cp ../../target/release/examples/iitiiri_query $@

bin/iitiiri_parallel_query_%: ../../examples/iitiiri_query.rs
	@mkdir -p bin
	@IITIIRI_DOMAIN=$* cargo build --release --example iitiiri_query --features parallel
	@cp ../../target/release/examples/iitiiri_query $@

bin/iitiiri_annotation_variant_%: ../../examples/iitiiri_annotation_variant.rs
	@mkdir -p bin
	@IITIIRI_DOMAIN=$* cargo build --release --example iitiiri_annotation_variant
	@cp ../../target/release/examples/iitiiri_annotation_variant $@

bin/iitiiri_parallel_annotation_variant_%: ../../examples/iitiiri_annotation_variant.rs
	@mkdir -p bin
	@IITIIRI_DOMAIN=$* cargo build --release --example iitiiri_annotation_variant --features parallel
	@cp ../../target/release/examples/iitiiri_annotation_variant $@

# Benchmark rule
result/build_%.csv: dataset/%.bed bin/cgranges_build bin/iitii_build bin/rust_bio_build bin/iitri_build bin/iitri_parallel_build $(iitiiri_build_path) $(iitiiri_build_p_path)
	@mkdir -p result
	@hyperfine --export-csv $@ \
	-n cgranges \
	$(iitii_hyper_name) \
	-n rust_bio \
	-n iitri -n iitri_parallel \
	$(iitiiri_hyper_name) \
	$(iitiiri_hyper_p_name) \
	"./bin/cgranges_build dataset/$*.bed" \
	$(iitii_build_hyper_cmd) \
	"./bin/rust_bio_build dataset/$*.bed" \
	"./bin/iitri_build dataset/$*.bed" "./bin/iitri_parallel_build dataset/$*.bed" \
	$(iitiiri_build_hyper_cmd) \
	$(iitiiri_build_hyper_p_cmd)

result/query_%.csv: dataset/%.bed bin/cgranges_query bin/iitii_query bin/rust_bio_query bin/iitri_query bin/iitri_parallel_query $(iitiiri_query_path) $(iitiiri_query_p_path)
	@mkdir -p result
	@echo "command,index,time" > $@
	@./bin/cgranges_query dataset/$*.bed >> $@
	@for d in $(domain) ; do \
	./bin/iitii_query dataset/$*.bed $$d >> $@ ; \
	done
	@./bin/rust_bio_query dataset/$*.bed >> $@
	@./bin/iitri_query dataset/$*.bed >> $@
	@./bin/iitri_parallel_query dataset/$*.bed >> $@
	@for d in $(domain) ; do \
	./bin/iitiiri_query_$$d dataset/$*.bed >> $@ ; \
	done
	@for d in $(domain) ; do \
	./bin/iitiiri_parallel_query_$$d dataset/$*.bed >> $@ ; \
	done

result/annotation_variant_%.csv: dataset/hg38.bed dataset/%.bed bin/cgranges_annotation_variant bin/iitii_annotation_variant $(iitiiri_annotation_variant_path) $(iitiiri_annotation_variant_p_path)
	@mkdir -p result
	@hyperfine --export-csv $@ \
	-n cgranges \
	$(iitii_hyper_name) \
	$(iitiiri_hyper_name) \
	$(iitiiri_hyper_p_name) \
	"./bin/cgranges_annotation_variant dataset/hg38.bed dataset/$*.bed" \
	$(iitii_annotation_variant_hyper_cmd) \
	$(iitiiri_annotation_variant_hyper_cmd) \
	$(iitiiri_annotation_variant_hyper_p_cmd) \

result/thread_effect_%.csv: dataset/%.bed $(iitiiri_build_path) $(iitiiri_build_p_path)
	@mkdir -p result
	@hyperfine --export-csv $@ \
	$(thread_effect_hyper_name_0) \
	$(thread_effect_hyper_name_1) $(thread_effect_hyper_name_2) $(thread_effect_hyper_name_3) $(thread_effect_hyper_name_4) $(thread_effect_hyper_name_5) $(thread_effect_hyper_name_6) $(thread_effect_hyper_name_7) $(thread_effect_hyper_name_8) \
	$(thread_effect_hyper_cmd_0) \
	$(thread_effect_hyper_cmd_1) $(thread_effect_hyper_cmd_2) $(thread_effect_hyper_cmd_3) $(thread_effect_hyper_cmd_4) $(thread_effect_hyper_cmd_5) $(thread_effect_hyper_cmd_6) $(thread_effect_hyper_cmd_7) $(thread_effect_hyper_cmd_8) \


benchmark_build: result/build_clinvar.csv result/build_gnomad.csv result/build_hg38.csv result/build_hg001.csv result/build_hg002.csv result/build_hg003.csv result/build_hg004.csv result/build_hg006.csv result/build_hg007.csv result/build_combine_shuffle.csv

benchmark_query: result/query_clinvar.csv result/query_gnomad.csv result/query_hg38.csv result/query_hg001.csv result/query_hg002.csv result/query_hg003.csv result/query_hg004.csv result/query_hg006.csv result/query_hg007.csv result/query_combine_shuffle.csv

benchmark_annotation_variant: result/annotation_variant_clinvar.csv result/annotation_variant_gnomad.csv result/annotation_variant_hg001.csv result/annotation_variant_hg002.csv result/annotation_variant_hg003.csv result/annotation_variant_hg004.csv result/annotation_variant_hg006.csv result/annotation_variant_hg007.csv

benchmark_thread_effect: result/thread_effect_clinvar.csv result/thread_effect_gnomad.csv result/thread_effect_hg001.csv result/thread_effect_hg002.csv result/thread_effect_hg003.csv result/thread_effect_hg004.csv result/thread_effect_hg006.csv result/thread_effect_hg007.csv result/thread_effect_combine_shuffle.csv


benchmark: benchmark_build benchmark_query benchmark_annotation_variant benchmark_thread_effect

# Util rule
clean: clean_data clean_dependency clean_bin clean_result

clean_dependency:
	@rm -rf cgranges iitii

clean_data:
	@rm -rf dataset

clean_bin:
	@rm -rf bin

clean_result:
	@rm -rf result

.PHONY: clean
