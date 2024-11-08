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


rule coitrees_bin:
    input:
        src = examples_path / "coitrees_{type}.rs"
    output:
        bin = "bin/coitrees_{type}"
    params:
        examples_path = lambda wcd: examples_path
    log:
        out = "log/bin/coitrees/{type}.stdout",
        err = "log/bin/coitrees/{type}.stderr",
    shell:
        """
        cargo build --release --example coitrees_{wildcards.type} 1> {log.out} 2> {log.err}
        cp {params.examples_path}/../target/release/examples/coitrees_{wildcards.type} {output.bin} 1> {log.out} 2> {log.err}
        """


rule clairiere_bin:
    input:
        src = examples_path / "clairiere_{type}.rs"
    output:
        bin = "bin/clairiere_{type}"
    params:
        examples_path = lambda wcd: examples_path
    log:
        out = "log/bin/clairiere/{type}.stdout",
        err = "log/bin/clairiere/{type}.stderr",
    shell:
        """
        cargo build --release --example clairiere_{wildcards.type} 1> {log.out} 2> {log.err}
        cp {params.examples_path}/../target/release/examples/clairiere_{wildcards.type} {output.bin} 1> {log.out} 2> {log.err}
        """


rule clairiere_parallel_bin:
    input:
        src = examples_path / "clairiere_{type}.rs"
    output:
        bin = "bin/clairiere_parallel_{type}"
    params:
        examples_path = lambda wcd: examples_path
    log:
        out = "log/bin/clairiere/{type}.stdout",
        err = "log/bin/clairiere/{type}.stderr",
    shell:
        """
        cargo build --release --features parallel --example clairiere_{wildcards.type} 1> {log.out} 2> {log.err}
        cp {params.examples_path}/../target/release/examples/clairiere_{wildcards.type} {output.bin} 1> {log.out} 2> {log.err}
        """


rule clairiere_interpolate_bin:
    input:
        src = examples_path / "clairiere_interpolate_{type}.rs"
    output:
        bin = "bin/clairiere_interpolate_{type}_{domain}"
    params:
        examples_path = lambda wcd: examples_path
    log:
        out = "log/bin/clairiere_interpolate/{type}_{domain}.stdout",
        err = "log/bin/clairiere_interpolate/{type}_{domain}.stderr",
    shell:
        """
        CLAIRIERE_DOMAIN={wildcards.domain} cargo build --release --example clairiere_interpolate_{wildcards.type} 1> {log.out} 2> {log.err}
        cp {params.examples_path}/../target/release/examples/clairiere_interpolate_{wildcards.type} {output.bin} 1> {log.out} 2> {log.err}
        """


rule clairiere_interpolate_parallel_bin:
    input:
        src = examples_path / "clairiere_interpolate_{type}.rs"
    output:
        bin = "bin/clairiere_interpolate_parallel_{type}_{domain}"
    params:
        examples_path = lambda wcd: examples_path
    log:
        out = "log/bin/clairiere_interpolate/{type}_{domain}.stdout",
        err = "log/bin/clairiere_interpolate/{type}_{domain}.stderr",
    shell:
        """
        CLAIRIERE_DOMAIN={wildcards.domain} cargo build --release --features parallel --example clairiere_interpolate_{wildcards.type} 1> {log.out} 2> {log.err}
        cp {params.examples_path}/../target/release/examples/clairiere_interpolate_{wildcards.type} {output.bin} 1> {log.out} 2> {log.err}
        """
