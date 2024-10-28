rule generic_data:
    output:
        bed = "data/{name}.bed"
    params:
        uri = lambda wcd: config["paths"]["data"][wcd.name]["uri"]
    log:
        out = "log/data/{name}.stdout",
        err = "log/data/{name}.stderr",
    shell:
        """
        curl {params.uri} 2> {log.err} | \
        gunzip - 2> {log.err} | \
        grep -v "^#" 2> {log.err} | \
        cut -f 1,2,5 2> {log.err} | \
        awk '{{print $1,$2,$2+length($3)}}' > {output.bed} 2> {log.err}
        """

rule hg38_data:
    output:
        bed = "data/hg38.bed"
    params:
        uri = lambda wcd: config["paths"]["data"]["hg38"]["uri"]
    log:
        out = "log/data/hg38.stdout",
        err = "log/data/hg38.stderr",
    shell:
        """
	curl {params.uri} 2> {log.err} | \
        gunzip - 2> {log.err} | \
        grep -v "^#" 2> {log.err} | \
        cut -f 1,4,5 2> {log.err} | \
        tr '\t' ' ' > {output.bed} 2> {log.err}
        """

rule all_variants:
    input:
        variants = lambda _wcd: [f"data/{dataset}.bed" for dataset in config["paths"]["data"]]
    output:
        bed = "data/all_variants.bed"
    log:
        err = "log/data/all_variants.stderr",
    shell:
        """
        cat {input.variants} 1> {output.bed} 2> {log.err}
        """
