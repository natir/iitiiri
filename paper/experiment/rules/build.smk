rule dependencies:
    output:
        directory("dependencies/{name}/")
    params:
        uri = lambda wcd: config["paths"]["dependencies"][wcd.name]["uri"],
        version = lambda wcd: config["paths"]["dependencies"][wcd.name]["version"],
    log:
        out = "log/dependencies/{name}.stdout",
        err = "log/dependencies/{name}.stderr",
    shell:
        """
        curl -L {params.uri} 2> {log.err} | \
        tar xvz 1> {log.out} 2> {log.err}
        mv {wildcards.name}-{params.version} {output} 2> {log.err}
        """


rule cgranges_bin:
    input:
        src = sources_path / "cgranges_{type}.c",
        dep = "dependencies/cgranges/",
    output:
        bin = "bin/cgranges_{type}"
    log:
        out = "log/bin/cgranges/{type}.stdout",
        err = "log/bin/cgranges/{type}.stderr",
    shell:
        """
        cc -O2 -DNDEBUG -I {input.dep} {input.dep}/cgranges.c {input.src} -o {output.bin} 1> {log.out} 2> {log.err}
        """


rule iitii_bin:
    input:
        src = sources_path / "iitii_{type}.cpp",
        dep = "dependencies/iitii/",
    output:
        bin = "bin/iitii_{type}"
    log:
        out = "log/bin/iitii/{type}.stdout",
        err = "log/bin/iitii/{type}.stderr",
    shell:
        """
        c++ -O2 -DNDEBUG -I {input.dep} {input.src} -o {output.bin} 1> {log.out} 2> {log.err}
        """


rule rust_bio_bin:
    input:
        src = examples_path / "rust_bio_{type}.rs"
    output:
        bin = "bin/rust_bio_{type}"
    params:
        examples_path = lambda wcd: examples_path
    log:
        out = "log/bin/rust_bio/{type}.stdout",
        err = "log/bin/rust_bio/{type}.stderr",
    shell:
        """
        cargo build --release --example rust_bio_{wildcards.type} 1> {log.out} 2> {log.err}
        cp {params.examples_path}/../target/release/examples/rust_bio_{wildcards.type} {output.bin} 1> {log.out} 2> {log.err}
        """


rule iitri_bin:
    input:
        src = examples_path / "iitri_{type}.rs"
    output:
        bin = "bin/iitri_{type}"
    params:
        examples_path = lambda wcd: examples_path
    log:
        out = "log/bin/iitri/{type}.stdout",
        err = "log/bin/iitri/{type}.stderr",
    shell:
        """
        cargo build --release --example iitri_{wildcards.type} 1> {log.out} 2> {log.err}
        cp {params.examples_path}/../target/release/examples/iitri_{wildcards.type} {output.bin} 1> {log.out} 2> {log.err}
        """


rule iitri_parallel_bin:
    input:
        src = examples_path / "iitri_{type}.rs"
    output:
        bin = "bin/iitri_parallel_{type}"
    params:
        examples_path = lambda wcd: examples_path
    log:
        out = "log/bin/iitri/{type}.stdout",
        err = "log/bin/iitri/{type}.stderr",
    shell:
        """
        cargo build --release --features parallel --example iitri_{wildcards.type} 1> {log.out} 2> {log.err}
        cp {params.examples_path}/../target/release/examples/iitri_{wildcards.type} {output.bin} 1> {log.out} 2> {log.err}
        """


rule iitiiri_bin:
    input:
        src = examples_path / "iitiiri_{type}.rs"
    output:
        bin = "bin/iitiiri_{type}_{domain}"
    params:
        examples_path = lambda wcd: examples_path
    log:
        out = "log/bin/iitiiri/{type}_{domain}.stdout",
        err = "log/bin/iitiiri/{type}_{domain}.stderr",
    shell:
        """
        IITIIRI_DOMAIN={wildcards.domain} cargo build --release --example iitiiri_{wildcards.type} 1> {log.out} 2> {log.err}
        cp {params.examples_path}/../target/release/examples/iitiiri_{wildcards.type} {output.bin} 1> {log.out} 2> {log.err}
        """


rule iitiiri_parallel_bin:
    input:
        src = examples_path / "iitiiri_{type}.rs"
    output:
        bin = "bin/iitiiri_parallel_{type}_{domain}"
    params:
        examples_path = lambda wcd: examples_path
    log:
        out = "log/bin/iitiiri/{type}_{domain}.stdout",
        err = "log/bin/iitiiri/{type}_{domain}.stderr",
    shell:
        """
        IITIIRI_DOMAIN={wildcards.domain} cargo build --release --features parallel --example iitiiri_{wildcards.type} 1> {log.out} 2> {log.err}
        cp {params.examples_path}/../target/release/examples/iitiiri_{wildcards.type} {output.bin} 1> {log.out} 2> {log.err}
        """
