import subprocess
import os


def cd_and_call_maturin_build():
    #  Based on current folder, move to the correct folder
    prefix = ""
    # we are in root folder
    if os.path.isfile("README.md"):
        prefix += "cd elo_compnat && "
    # we are in elo_compnat/python folder
    elif os.path.isfile("main.py"):
        prefix += "cd ../ && "

    result = subprocess.run(
        f"{prefix} maturin build --release -i python", shell=True, capture_output=True
    )

    # Decode the output using the appropriate character encoding
    stderr = result.stderr.decode("utf-8", "replace")
    stdout = result.stdout.decode("utf-8", "replace")

    # Print the output of the command
    print(stdout)
    print(stderr)


cd_and_call_maturin_build()
