binary = ["clairiere", "clairiere_parallel", "clairiere_interpolate", "clairiere_interpolate_parallel"]

thread_path = []
for name in binary:
    if name.startswith("clairiere_interpolate"):
        for domain in config["variables"]["domains"]:
            thread_path.append(f"bin/{name}_build_{domain}")
    else:
        thread_path.append(f"bin/{name}_build")

thread_cmd = []
for name in binary:
    if name.startswith("clairiere_interpolate"):
        for domain in config["variables"]["domains"]:
            if name.endswith("parallel"):
                for thread in config["variables"]["threads"]:
                    thread_cmd.append(f"RAYON_NUM_THREADS={thread} ./bin/{name}_build_{domain} {{dataset}}")
            else:
                thread_cmd.append(f"./bin/{name}_build_{domain} {{dataset}}")
    else:
        if name.endswith("parallel"):
            for thread in config["variables"]["threads"]:
                thread_cmd.append(f"RAYON_NUM_THREADS={thread} ./bin/{name}_build {{dataset}}")
        else:
            thread_cmd.append(f"./bin/{name}_build {{dataset}}")

hyperfine_name = []
for name in binary:
    if name.startswith("clairiere_interpolate"):
        for domain in config["variables"]["domains"]:
            if name.endswith("parallel"):
                for thread in config["variables"]["threads"]:
                    hyperfine_name.append(f"-n {name}_build_{domain}_{thread}")
            else:
                hyperfine_name.append(f"-n {name}_build_{domain}")
    else:
        if name.endswith("parallel"):
            for thread in config["variables"]["threads"]:
                hyperfine_name.append(f"-n {name}_build_{thread}")
        else:
            hyperfine_name.append(f"-n {name}_build")


rule thread_effect:
    input:
        data = "data/{dataset}.bed",
        bin = thread_path,
    output:
        result = "thread/{dataset}.csv"
    params:
        name = lambda _wcd: " ".join(hyperfine_name),
        cmd = lambda wcd, input: " ".join([f"'{cmd.format(dataset=input.data)}'" for cmd in thread_cmd])
    threads:
        2**63
    log:
        out = "log/thread/{dataset}.stdout",
        err = "log/thread/{dataset}.stderr",
    shell:
        """
        hyperfine --export-csv {output.result} \
        {params.name} \
        {params.cmd} \
        1> {log.out} 2> {log.err}
        """
