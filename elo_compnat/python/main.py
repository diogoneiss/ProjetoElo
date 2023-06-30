import elo_compnat
import numpy as np
import pygad

from cache import insert_document

# todo: parametrizar isso aqui para filtrar pelo nome do dataset
partidas = elo_compnat.get_data()

RunHyperparameters = elo_compnat.RunHyperparameters
RunConfig = elo_compnat.RunConfig

# consultar a funcao pra ver a ordem e oq cada um é
hyperparams_list = [4444, 2003, 8, 20, 1, 0, 0, 1]

# vai criar um objeto com os parametros e retornar a partir da lista
# poderiamos fazer um mapping de dict <-> RunHyperparameters, mas da na mesma
# vamos precisar fazer isso também para os parâmetros do GA (RunConfig)
hiperparametros_obj = RunHyperparameters.from_list(hyperparams_list)

print(hiperparametros_obj.__dict__)

genotype_list = [0.1, 0.2, 0.3, 0.4, 0.5, 0.6]
run_config_obj = RunConfig.from_list(genotype_list)


err = elo_compnat.fitness_function(partidas, run_config_obj, hiperparametros_obj)
print("processou os dados\n\n\n")
elo_compnat.fitness_function(partidas, run_config_obj, hiperparametros_obj)
print("processou os dados\n\n\n")

# usaremos isso aqui pra salvar em nuvem os resultados
# insert_document()


# elo_compnat.run(hiperparametros_obj)

function_inputs = [4, -2, 3.5, 5, -11, -4.7]
desired_output = 44


def fitness_func(ga_instance, solution, solution_idx):
    output = np.sum(solution * function_inputs)
    fitness = 1.0 / np.abs(output - desired_output)
    return fitness


fitness_function = fitness_func

num_generations = 50
num_parents_mating = 4

sol_per_pop = 80
num_genes = len(function_inputs)

init_range_low = -2
init_range_high = 5

elitism_percentage = 0.1
keep_elitism = max(1, int(elitism_percentage * sol_per_pop))

parent_selection_type = "sss"
"""
Parent_selection_type="sss": The parent selection type. 
Supported types are 
sss (for steady-state selection), 
rws (for roulette wheel selection), 
sus (for stochastic universal selection), 
rank (for rank selection), 
random (for random selection), and 
tournament (for tournament selection). 

A custom parent selection function can be passed starting from PyGAD 2.16.0. 
Check the User-Defined Crossover, Mutation, and Parent Selection Operators section for more details about building a user-defined parent selection function.
"""

keep_parents = 1

crossover_type = "single_point"
crossover_probability = 0.2
"""
crossover_type="single_point": 
Type of the crossover operation. Supported types are 
single_point (for single-point crossover), 
two_points (for two points crossover), 
uniform (for uniform crossover), and 
scattered (for scattered crossover). Scattered crossover is supported from PyGAD 2.9.0 and higher. 

It defaults to single_point. 
A custom crossover function can be passed starting from PyGAD 2.16.0. 
Check the User-Defined Crossover, Mutation, and Parent Selection Operators section for more details about creating a user-defined crossover function.
Starting from PyGAD 2.2.2 and higher, if crossover_type=None, 
then the crossover step is bypassed which means no crossover is applied and thus no offspring will be created in the next generations. 
The next generation will use the solutions in the current population.
"""

mutation_type = "random"
mutation_probability = 0.1

# TODO: colocar limite nos parametros aqui
gene_space = None

save_best_solutions = True

# olhar melhor isso

parallel_processing = 10

random_seed = 42

ga_instance = pygad.GA(
    num_generations=num_generations,
    num_parents_mating=num_parents_mating,
    fitness_func=fitness_function,
    sol_per_pop=sol_per_pop,
    num_genes=num_genes,
    init_range_low=init_range_low,
    init_range_high=init_range_high,
    parent_selection_type=parent_selection_type,
    keep_parents=keep_parents,
    crossover_type=crossover_type,
    crossover_probability=crossover_probability,
    mutation_type=mutation_type,
    mutation_probability=mutation_probability,
    save_best_solutions=save_best_solutions,
    parallel_processing=parallel_processing,
    random_seed=random_seed,
)

# não usamos comp nat ainda

ga_instance.run()


solution, solution_fitness, solution_idx = ga_instance.best_solution()
prediction = np.sum(np.array(function_inputs) * solution)

print(f"Parameters of the best solution : {solution}")
print(f"Fitness value of the best solution = {solution_fitness}")
print(f"Predicted output based on the best solution : {prediction}")
