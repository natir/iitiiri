rule affine_effect:
    input:
        data = "data/{dataset}.bed",
    output:
        result = "affine_effect/{dataset}_{domain}.csv"
    threads:
        1
    log:
        err = "log/affine_effect/{dataset}_{domain}.stderr"
    shell:
        """
        IITIIRI_DOMAIN={wildcards.domain} cargo run --release --features eval_guess --example iitiiri_annotate -- {input.data} {input.data} 1> {output.result} 2> {log.err}
        """
