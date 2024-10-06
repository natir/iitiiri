memory_build_cmd = []
for name in config["variables"]["binaries"]:
    if name.startswith("iitiiri"):
        for domain in config["variables"]["domains"]:
            memory_build_cmd.append(
                f"/usr/bin/time -f '{name}_build_{domain},%M' ./bin/{name}_build_{domain} {{input}} 2>> {{output}}"
            )
    elif name.startswith("iitii"):
        for domain in config["variables"]["domains"]:
            memory_build_cmd.append(
                f"/usr/bin/time -f '{name}_build_{domain},%M' ./bin/{name}_build {{input}} {domain} 2>> {{output}}"
            )
    else:
        memory_build_cmd.append(
            f"/usr/bin/time -f '{name}_build,%M' ./bin/{name}_build {{input}} 2>> {{output}}"
            )


rule memory:
    input:
        data = "data/{name}.bed",
        bin = lambda wcd: [path for path in bin_path["build"]]
    output:
        result = "memory/{type}/{name}.csv"
    params:
        cmd = lambda wcd, input, output: " && ".join([line.format(input=input.data, output=output.result) for line in  memory_build_cmd])
    threads:
        2**63
    log:
        out = "log/memory/{type}/{name}.stdout",
        err = "log/memory/{type}/{name}.stderr",
    shell:
        """
        echo "command,memory" > {output.result} 2> {log.err} && {params.cmd}
        """
