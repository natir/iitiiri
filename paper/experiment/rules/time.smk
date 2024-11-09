hyperfine_name_time = {"build":[], "annotate":[]}
for target in hyperfine_name_time:
    for name in config["variables"]["binaries"]:
        if name.startswith("clairiere_interpolate"):
            for domain in config["variables"]["domains"]:
                hyperfine_name_time[target].append(f"-n {name}_{target}_{domain}")
        elif name.startswith("iitii"):
            for domain in config["variables"]["domains"]:
                hyperfine_name_time[target].append(f"-n {name}_{target}_{domain}")
        else:
            hyperfine_name_time[target].append(f"-n {name}_{target}")

rule time:
    input:
        data = "data/{dataset}.bed",
        bin = lambda wcd: [path for path in bin_path[wcd.target]],
    output:
        result = "time/{target}/{dataset}.csv"
    params:
        name = lambda wcd: " ".join(hyperfine_name_time[wcd.target]),
        cmd = lambda wcd, input: " ".join([f"'{cmd.format(dataset=input.data)}'" for cmd in bin_cmd[wcd.target]]),
    threads:
        2**63
    log:
        out = "log/time/{target}/{dataset}.stdout",
        err = "log/time/{target}/{dataset}.stderr",
    shell:
        """
        RAYON_NUM_THREADS=4
        hyperfine --export-csv {output.result} \
        {params.name} \
        {params.cmd} \
        1> {log.out} 2> {log.err}
        """


time_query_cmd = " && ".join([
    f"{cmd.format(dataset='{input.data}')} >> {{output.result}} 2>> {{log.err}}"
    for cmd in bin_cmd["query"]
])
rule time_query:
    input:
        data = "data/{dataset}.bed",
        bin = lambda wcd: [path for path in bin_path["query"]]
    output:
        result = "time/query/{dataset}.csv"
    threads:
        2**63
    log:
        out = "log/time/query/{dataset}.stdout",
        err = "log/time/query/{dataset}.stderr",
    shell:
        f"""
        RAYON_NUM_THREADS=14
        echo \"command,index,time\" > {{output.result}} 2> {{log.err}} && {time_query_cmd}
        """

ruleorder: time_query > time
