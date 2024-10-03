import argparse
import os
import subprocess
import re


# Number of runs for each benchmark
num_runs = 1

# Verbose output
verbose = True

# Time limit for each benchmark
time_limit = 60

FUNCTIONS = [ ("add", "{\"param1\":15,\"param2\":3}"),
              ("fib", "{\"param1\":30}"),
              ("noop", "{\"param1\":\"a\",\"param2\":\"b\",\"param3\":\"c\"}" ),
            ]



def compile(function):
    # Compile all benchmarks
    print("\n\n-----------------------------------------------")
    print("\033[92m", "\nCompiling", function, "\033[0m")

    function = function+".rs"

    # "build.sh $function" at ./inherit_stdio/rust_functions
    completed_process = subprocess.run(["./compile.sh", function], cwd="inherit_stdio/rust_functions", stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)
    if(verbose): print(f"\n\tstdout: {completed_process.stdout} \n\tstderr: {completed_process.stderr}")
    
    # "build.sh $function" at ./memory_export/rust_functions
    completed_process = completed_process = subprocess.run(["./compile.sh", function], cwd="memory_export/rust_functions", stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)
    if(verbose): print(f"\n\tstdout: {completed_process.stdout} \n\tstderr: {completed_process.stderr}")
    
    # "build_components.sh $function.rs" at ./component_model/rust_func
    completed_process = subprocess.run(["./build_component.sh", function], cwd="component_model/rust_functions", stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)
    if(verbose): print(f"\n\tstdout: {completed_process.stdout} \n\tstderr: {completed_process.stderr}")



def run_bench(command, embedder, function):
    result = ""

    for i in range(num_runs):

        print("\n", " ".join(command))

        completed_process=subprocess.run(command, stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True, timeout=time_limit)

        if(verbose): print(f"\n\tstdout: {completed_process.stdout} \n\tstderr: {completed_process.stderr}")
            
        # Extract the numerical values from the output
        numerical_values = ','.join(re.findall(r"(\d+\.\d+|\d+)", completed_process.stdout))

        # Add the real mean and real stddev to the result
        result += f"\n{embedder},{function},{numerical_values}"


    print (f"\n\tResult: {result}")

    return result




def main():
   
    # Create CSV file for current benchmark and thread number
    csv_file = f"result.csv"
    with open(csv_file, "w") as file:
        file.write("Embedder,Function,Deserialization_time,Preinstantiation_time,Instantiation_time,Execution_time,Result_retrieve_time,Total_time,Result")

        for function, payload in FUNCTIONS:
            compile(function)

            print("\n\n-----------------------------------------------")
            print("\033[92m", "\nRunning inherit_stdio", function, "\033[0m")
            command = ["cargo", "run", "--release", "--manifest-path", "inherit_stdio/Cargo.toml", f"inherit_stdio/rust_functions/compiled/{function}.cwasm", payload]
            file.write(run_bench(command, "inherit_stdio", function))

            print("\n\n-----------------------------------------------")
            print("\033[92m", "\nRunning memory_export", function, "\033[0m")
            command = ["cargo", "run", "--release", "--manifest-path", "memory_export/Cargo.toml", f"memory_export/rust_functions/compiled/{function}.cwasm", payload]
            file.write(run_bench(command, "memory_export", function))
            
            print("\n\n-----------------------------------------------")
            print("\033[92m", "\nRunning component_model", function, "\033[0m")
            command = ["cargo", "run", "--release", "--manifest-path", "component_model/Cargo.toml", f"component_model/rust_functions/compiled/{function}.cwasm", payload]
            file.write(run_bench(command, "component_model", function))

    
                
    

# Parse the input arguments
def parse_arguments():
    global num_runs, verbose
    parser = argparse.ArgumentParser(description='Arguments for benchmarking')

    parser.add_argument('-n', '--num_runs', type=int, default=num_runs,
                        help='Number of runs for each benchmark (default: {})'.format(num_runs))
    
    parser.add_argument('-v', '--verbose', action='store_true', default=verbose,
                        help='Enable verbose output (default: {})'.format(verbose))


    args = parser.parse_args()

    num_runs = args.num_runs
    verbose = args.verbose



if __name__ == '__main__':
    parse_arguments()
    main()