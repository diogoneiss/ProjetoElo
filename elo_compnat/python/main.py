import elo_compnat
import numpy as np
import pygad
import matplotlib.pyplot as plt
from cache import insert_document

# todo: parametrizar isso aqui para filtrar pelo nome do dataset
partidas = elo_compnat.get_data()

RunHyperparameters = elo_compnat.RunHyperparameters
RunConfig = elo_compnat.RunConfig
CustomRating = elo_compnat.CustomRating
CustomElo = elo_compnat.CustomElo

# consultar a funcao pra ver a ordem e oq cada um é
# starting_elo, starting_year, backtest_years, random-variations, use_goals_diff, use_home_advantage, use_market_value, leagues_to_use
hyperparams_list = [1000, 2003, 8, 100, 0, 0, 0, 1]

# vai criar um objeto com os parametros e retornar a partir da lista
# poderiamos fazer um mapping de dict <-> RunHyperparameters, mas da na mesma
# vamos precisar fazer isso também para os parâmetros do GA (RunConfig)
hiperparametros_obj = RunHyperparameters.from_list(hyperparams_list)

print(hiperparametros_obj.__dict__)

w_division = [40, 20]
genotype_list = [40, 1, 1, 0.0075, 1, 0.5, 0.5, *w_division]

run_config_obj = RunConfig.from_list(genotype_list)

posicao_parametros_runconfig = {'k_factor': 0,
                                'gamma': 1,
                                'home_advantage': 2,
                                'home_field_advantage_weight': 3,
                                'market_value_weight': 4,
                                'tie_frequency': 5,
                                'w_division': 6}

# https://pygad.readthedocs.io/en/latest/pygad.html#more-about-the-gene-space-parameter

k_factor = {'low': 10,
            'high': 60,
            "step": 1.0
            }

gamma = {'low': 0.3, 'high': 2}
home_advantage = {'low': 0, 'high': 2}
home_field_advantage_weight = {'low': 0, 'high': 1}
market_value_weight = {'low': 0, 'high': 1}
tie_frequency = {'low': 0, 'high': 1}

w_division = [{'low': 10, 'high': 80}, {'low': 10, 'high': 80}]

# todo: fazer o gene space ser um dict com os parametros e os valores
gene_space = [k_factor, gamma, home_advantage, home_field_advantage_weight,
              market_value_weight, tie_frequency, w_division[0], w_division[1]]


err = elo_compnat.fitness_function(
    partidas, run_config_obj, hiperparametros_obj)

experiment_start_year = hyperparams_list[1]+hyperparams_list[2]
plot = False

if plot:
    x = np.arange(experiment_start_year, experiment_start_year+len(err), 1)

    plt.plot(x, err)
    plt.title("Erros do modelo")
    plt.xlabel(f"Temporada simulada")
    plt.show()



# usaremos isso aqui pra salvar em nuvem os resultados
# insert_document()


# elo_compnat.run(hiperparametros_obj)

function_inputs = [4, -2, 3.5, 5, -11, -4.7]
desired_output = 44


def fitness_func(ga_instance, solution, solution_idx):
    # global parameters, we dont change them
    global partidas, hiperparametros_obj
    print(solution)
    run_config_obj = RunConfig.from_list(solution)
    print(run_config_obj.__dict__)

    err = elo_compnat.fitness_function(partidas, run_config_obj, hiperparametros_obj)
    fitness = np.divide(1, np.sum(np.abs(err)))
    return fitness


fitness_function = fitness_func

num_generations = 50
num_parents_mating = 4

sol_per_pop = 8
num_genes = len(gene_space)


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

mutation_type = "adaptive"
# todo: variar isso
mutation_probability = [0.25, 0.1]

save_best_solutions = True

# olhar melhor isso

parallel_processing = 4

random_seed = 42

ga_instance = pygad.GA(
    num_generations=num_generations,
    num_parents_mating=num_parents_mating,
    fitness_func=fitness_function,
    sol_per_pop=sol_per_pop,
    num_genes=num_genes,
    parent_selection_type=parent_selection_type,
    keep_parents=keep_parents,
    crossover_type=crossover_type,
    crossover_probability=crossover_probability,
    mutation_type=mutation_type,
    mutation_probability=mutation_probability,
    save_best_solutions=save_best_solutions,
    parallel_processing=parallel_processing,
    allow_duplicate_genes=True,
    gene_space=gene_space,
    random_seed=random_seed,
)

# não usamos comp nat ainda

ga_instance.run()


solution, solution_fitness, solution_idx = ga_instance.best_solution()

# todo: montar tabela de elo aqui
prediction = solution

print(f"Parameters of the best solution : {solution}")
print(f"Fitness value of the best solution = {solution_fitness}")
print(f"Predicted output based on the best solution : {prediction}")
