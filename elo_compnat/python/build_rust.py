import subprocess

def cd_and_call_maturin_develop():
    # Call "maturin develop" in the current terminal
    result = subprocess.run('cd ../ && maturin develop', shell=True, capture_output=True)
    
    # Decode the output using the appropriate character encoding
    stdout = result.stdout.decode('utf-8')
    stderr = result.stderr.decode('utf-8')
    
    # Print the output of the command
    print(stdout)
    print(stderr)
    
cd_and_call_maturin_develop()
