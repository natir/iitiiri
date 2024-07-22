cgranges_version = 0.1.1
iitii_version = bc049182fc6aecf5b48316bca9bd57785673782d
clinvar_version = 20240730

all: benchmark


# Get data
gnomad.bed:
	@curl https://storage.googleapis.com/gcp-public-data--gnomad/release/2.1.1/vcf/exomes/gnomad.exomes.r2.1.1.sites.2.vcf.bgz | gunzip - | grep -v "^#" | cut -f 1,2,5 |  awk '{print $$1,$$2,$$2+length($$3)}' > gnomad.bed

clinvar.bed:
	@curl https://ftp.ncbi.nlm.nih.gov/pub/clinvar/vcf_GRCh38/archive_2.0/2024/clinvar_$(clinvar_version).vcf.gz | gunzip - | grep -v "^#" | cut -f 1,2,5 |  awk '{print $$1,$$2,$$2+length($$3)}' > clinvar.bed

hg38.bed:
	@curl https://ftp.ensembl.org/pub/release-92/gff3/homo_sapiens/Homo_sapiens.GRCh38.92.gff3.gz | gunzip - | grep -v "^#" | cut -f 1,4,5 | tr '\t' ' ' > hg38.bed

combine_shuffle.bed: gnomad.bed clinvar.bed hg38.bed
	@cat gnomad.bed clinvar.bed hg38.bed | shuf --random-source gnomad.bed > combine_shuffle.bed


# Build cgranges
cgranges:
	@curl -L https://github.com/lh3/cgranges/archive/refs/tags/v$(cgranges_version).tar.gz | tar xvz
	@mv cgranges-$(cgranges_version) cgranges

cgranges_build: cgranges cgranges_build.c
	@cc -O2 -DNDEBUG cgranges/cgranges.c cgranges_build.c -o cgranges_build

cgranges_query: cgranges cgranges_query.c
	@cc -O2 -DNDEBUG cgranges/cgranges.c cgranges_query.c -o cgranges_query


# Build iitii
iitii:
	@curl -L https://github.com/mlin/iitii/archive/$(iitii_version).tar.gz | tar xvz
	@mv iitii-$(iitii_version) iitii

iitii_build: iitii iitii_build.cpp
	@c++ -O2 -DNDEBUG iitii_build.cpp -o iitii_build

iitii_query: iitii iitii_query.cpp
	@c++ -O2 -DNDEBUG iitii_query.cpp -o iitii_query


# Build iitri
iitri_build: ../../examples/iitri_build.rs
	@cargo build --release --example iitri_build
	@cp ../../target/release/examples/iitri_build iitri_build

iitri_parallel_build: ../../examples/iitri_build.rs
	@cargo build --release --example iitri_build --features parallel
	@cp ../../target/release/examples/iitri_build iitri_parallel_build

iitri_query: ../../examples/iitri_query.rs
	@cargo build --release --example iitri_query
	@cp ../../target/release/examples/iitri_query iitri_query

iitri_parallel_query: ../../examples/iitri_query.rs
	@cargo build --release --example iitri_query --features parallel
	@cp ../../target/release/examples/iitri_query iitri_parallel_query


# Build iitiiri
iitiiri_build_%: ../../examples/iitiiri_build.rs
	@IITIIRI_DOMAIN=$* cargo build --release --example iitiiri_build
	@cp ../../target/release/examples/iitiiri_build $@

iitiiri_parallel_build_%: ../../examples/iitiiri_build.rs
	@IITIIRI_DOMAIN=$* cargo build --release --example iitiiri_build --features parallel
	@cp ../../target/release/examples/iitiiri_build $@

iitiiri_query_%: ../../examples/iitiiri_query.rs
	@IITIIRI_DOMAIN=$* cargo build --release --example iitiiri_query
	@cp ../../target/release/examples/iitiiri_query $@

iitiiri_parallel_query_%: ../../examples/iitiiri_query.rs
	@IITIIRI_DOMAIN=$* cargo build --release --example iitiiri_query --features parallel
	@cp ../../target/release/examples/iitiiri_query $@


# Benchmark rule
%_build.csv: %.bed cgranges_build iitii_build iitri_build iitri_parallel_build iitiiri_build_2 iitiiri_build_4 iitiiri_build_8 iitiiri_build_16 iitiiri_parallel_build_2 iitiiri_parallel_build_4 iitiiri_parallel_build_8 iitiiri_parallel_build_16
	@hyperfine --export-csv $@ \
	-n cgranges \
	-n iitii_2 -n iitii_4 -n iitii_8 -n iitii_16 \
	-n iitri -n iitri_parallel \
	-n iitiiri_2 -n iitiiri_4 -n iitiiri_8 -n iitiiri_16 \
	-n iitiiri_parallel_2 -n iitiiri_parallel_4 -n iitiiri_parallel_8 -n iitiiri_parallel_16 \
	"./cgranges_build $*.bed" \
	"./iitii_build $*.bed 1" "./iitii_build $*.bed 2" "./iitii_build $*.bed 4" "./iitii_build $*.bed 8" \
	"./iitri_build $*.bed" "./iitri_parallel_build $*.bed" \
	"./iitiiri_build_2 $*.bed" "./iitiiri_build_4 $*.bed" "./iitiiri_build_8 $*.bed" "./iitiiri_build_16 $*.bed" \
	"./iitiiri_parallel_build_2 $*.bed" "./iitiiri_parallel_build_4 $*.bed" "./iitiiri_parallel_build_8 $*.bed" "./iitiiri_parallel_build_16 $*.bed" \

%_query.csv: %.bed cgranges_query iitii_query iitri_query iitri_parallel_query iitiiri_query_2 iitiiri_query_4 iitiiri_query_8 iitiiri_query_16 iitiiri_parallel_query_2 iitiiri_parallel_query_4 iitiiri_parallel_query_8 iitiiri_parallel_query_16
	@hyperfine --export-csv $@ \
	-n cgranges \
	-n iitii_2 -n iitii_4 -n iitii_8 -n iitii_16 \
	-n iitri -n iitri_parallel \
	-n iitiiri_2 -n iitiiri_4 -n iitiiri_8 -n iitiiri_16 \
	-n iitiiri_parallel_2 -n iitiiri_parallel_4 -n iitiiri_parallel_8 -n iitiiri_parallel_16 \
	"./cgranges_query $*.bed" \
	"./iitii_query $*.bed 1" "./iitii_query $*.bed 2" "./iitii_query $*.bed 4" "./iitii_query $*.bed 8" \
	"./iitri_query $*.bed" "./iitri_parallel_query $*.bed" \
	"./iitiiri_query_2 $*.bed" "./iitiiri_query_4 $*.bed" "./iitiiri_query_8 $*.bed" "./iitiiri_query_16 $*.bed" \
	"./iitiiri_parallel_query_2 $*.bed" "./iitiiri_parallel_query_4 $*.bed" "./iitiiri_parallel_query_8 $*.bed" "./iitiiri_parallel_query_16 $*.bed" \


benchmark_build: clinvar_build.csv gnomad_build.csv hg38_build.csv combine_shuffle_build.csv

benchmark_query: clinvar_query.csv gnomad_query.csv hg38_query.csv combine_shuffle_query.csv

benchmark: benchmark_build benchmark_query

# Util rule
clean: clean_data clean_dependency clean_bin clean_result

clean_dependency:
	@rm -rf cgranges iitii

clean_data:
	@rm -rf gnomad.bed clinvar.bed hg38.bed combine_shuffle.bed

clean_bin:
	@rm -rf cgranges_build
	@rm -rf cgranges_query
	@rm -rf iitii_build
	@rm -rf iitii_query
	@rm -rf iitri_build iitri_parallel_build
	@rm -rf iitri_query iitri_parallel_query
	@rm -rf iitiiri_build_2 iitiiri_build_4 iitiiri_build_8 iitiiri_build_16
	@rm -rf iitiiri_query_2 iitiiri_query_4 iitiiri_query_8 iitiiri_query_16

clean_result:
	@rm -rf clinvar_build.csv gnomad_build.csv hg38_build.csv combine_shuffle_build.csv
	@rm -rf clinvar_query.csv gnomad_query.csv hg38_query.csv combine_shuffle_query.csv

.PHONY: clean
