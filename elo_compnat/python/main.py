import elo_compnat
import numpy as np
import pygad
import matplotlib.pyplot as plt
from cache import insert_document
import pyswarms as ps
import pyswarms.backend.topology as topologies
import time
from pyswarms.utils.plotters import (plot_cost_history, plot_contour, plot_surface)
from pyswarms.utils.plotters.formatters import Mesher
from pyswarms.utils.search import RandomSearch
CORES = 10



# TODO: mover pro rust e só passar o caminho do arquivo

RunHyperparameters = elo_compnat.RunHyperparameters

RunConfig = elo_compnat.RunConfig
CustomRating = elo_compnat.CustomRating
CustomElo = elo_compnat.CustomElo
hyperparams_list = [1000, 2003, 8, 100, 1, 1, 1, 1]
start_time = time.perf_counter()

# vai criar um objeto com os parametros e retornar a partir da lista
# poderiamos fazer um mapping de dict <-> RunHyperparameters, mas da na mesma
# vamos precisar fazer isso também para os parâmetros do GA (RunConfig)
hiperparametros_obj = RunHyperparameters.from_list(hyperparams_list)

gene_space_dict = {
'k_factor': {'low': 1, 'high': 2},
'gamma': {'low': 0.3, 'high': 2},
'home_advantage': {'low': 0, 'high': 2},
'home_field_advantage_weight': {'low': 0, 'high': 1},
'market_value_weight': {'low': 0, 'high': 1},
# esse campo deve ser manualmente setado
#'tie_frequency': {'low': 0, 'high': 1},
'w_division_0': {'low': 10, 'high': 80},
'w_division_1': {'low': 10, 'high': 80}
}

"""
pub struct RunConfig {
    pub k_factor: f64,
    pub gamma: f64,
    pub home_advantage: f64,
    pub home_field_advantage_weight: f64,
    pub market_value_weight: f64,
    pub tie_frequency: f64,
    pub w_division: Vec<f64>,
}
"""

def inserir_frequencia(x, valor=0.28, posicao=5):
    return x[:posicao].tolist() + [valor] + x[posicao:].tolist()


def swarm_fitness_function(x_list_of_lists):
    aggregated_fitness = np.zeros(len(x_list_of_lists))

    for idx, x in enumerate(x_list_of_lists):


        # we have a x vector of dimension n, and need to create a config_list of size n+1, such that the 5th element is hard coded as 0.28
        position = 5
        # Create a config_list of size n+1
        config_list = inserir_frequencia(x)


        start = time.perf_counter()
        err = elo_compnat.fitness_function("brasileirao", config_list, hyperparams_list)
        fitness = np.sum(np.abs(err))

        print("Fitness function time: ", time.perf_counter() - start, " for solution ", idx, " with fitness ", fitness)
        # Append the fitness to the aggregated list
        aggregated_fitness[idx] = fitness
    #print("Aggregated fitness: ", aggregated_fitness)
    #print(aggregated_fitness.shape)
    # Return the aggregated fitness list
    return aggregated_fitness

# resultado podre:     [ 0.00185522, -0.02959553,  0.83426157, -0.79967734, -0.0774037,   0.02039052  0.05995651]

def run_genetic_algo():
    w_division = [40, 20]
    genotype_list = [40, 1, 1, 0.0075, 1, 0.5, *w_division]

    posicao_parametros_runconfig = {'k_factor': 0,
                                    'gamma': 1,
                                    'home_advantage': 2,
                                    'home_field_advantage_weight': 3,
                                    'market_value_weight': 4,
                                    'tie_frequency': 5,
                                    'w_division': (6, 7)}

    # https://pygad.readthedocs.io/en/latest/pygad.html#more-about-the-gene-space-parameter

    k_factor = {'low': 10,
                'high': 60,
                "step": 1.0
                }


    

    err = elo_compnat.fitness_function(
        "brasileirao", genotype_list, hyperparams_list)


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

    fitness_function = fitness_func


    num_generations = 5
    num_parents_mating = 2

    sol_per_pop = 4
    num_genes = len(gene_space)
    print("Number of genes: ", num_genes)

    elitism_percentage = 0.1
    keep_elitism = max(1, int(elitism_percentage * sol_per_pop))

    parent_selection_type = "sss"

    keep_parents = 1

    crossover_type = "single_point"
    crossover_probability = 0.2

    mutation_type = "adaptive"
    # todo: variar isso
    mutation_probability = [0.25, 0.1]

    save_best_solutions = True

    # olhar melhor isso

    parallel_processing = None
    #parallel_processing = ["process", 50]
    random_seed = 42

    gene_space = [value for value in gene_space_dict.values()]

    ga_instance = pygad.GA(
        num_generations=num_generations,
        num_parents_mating=num_parents_mating,
        fitness_func=fitness_function,
        on_generation=on_gen,
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
        suppress_warnings=True
    )

    # não usamos comp nat ainda

    ga_instance.run()

    solution, solution_fitness, solution_idx = ga_instance.best_solution()

    # todo: montar tabela de elo aqui
    prediction = solution

    print(f"Parameters of the best solution : {solution}")
    print(f"Fitness value of the best solution = {solution_fitness}")
    print(f"Predicted output based on the best solution : {prediction}")
    ga_instance.plot_fitness()
    ga_instance.plot_new_solution_rate()
    ga_instance.plot_genes()

def fitness_func(ga_instance, solution, solution_idx):
    position = 5
    # Create a config_list of size n+1
    config_list = solution[:position].tolist() + [0.28] + solution[position:].tolist()
    # global parameters, we dont change them
    run_config_obj = RunConfig.from_list(config_list)
    # print(run_config_obj.__dict__)
    start = time.perf_counter()
    err = elo_compnat.fitness_function(
        "brasileirao", solution, hyperparams_list)
    fitness = np.divide(1, np.sum(np.abs(err)))
    print("Fitness function time: ", time.perf_counter() - start, " for solution ", solution_idx, " with fitness ", fitness)
    return fitness

def on_gen(ga_instance):
    print("Time taken {0} minutes".format(
        round((time.perf_counter() - start_time)/60, 2)))
    print("Generation : ", ga_instance.generations_completed)
    print("Fitness of the best solution :", ga_instance.best_solution()[1])

def main():

    #run_genetic_algo()


    # Extract the low and high values
    low_values = [value['low'] for value in gene_space_dict.values()]
    high_values = [value['high'] for value in gene_space_dict.values()]

    # Convert the lists to numpy arrays
    max_bound = np.array(low_values)
    min_bound = np.array(high_values)
    bounds = (min_bound, max_bound)

    # hyperparameters for PSO
    options = {'c1': 0.5, 'c2': 0.2, 'w': 0.9}
    my_topology = topologies.Ring(static=True)
    dimensions = len(gene_space_dict)
    n_particles = 10

    options = {'c1': [0.3, 2],
               'c2': [0.3, 2],
               'w' : [0.5, 2],
               'k' : [11, 15],
               'p' : 1}
    
    g = RandomSearch(ps.single.GlobalBestPSO, n_particles=40, dimensions=dimensions,
                   options=options, objective_func=swarm_fitness_function, iters=20,n_selection_iters=40 )
    
    best_score, best_options = g.search()
    print("Bounds: ", g.bounds)
    print("Best score:", best_score)
    print("Best options: ", best_options)

    # Call instance of GlobalBestPSO
    optimizer = ps.single.GlobalBestPSO(n_particles=10, dimensions=dimensions,
                                              options=options)
    

    
    cost, pos = optimizer.optimize(swarm_fitness_function,  n_processes=CORES, iters=20)

    print("Best cost: ", cost)
    print("Best position: ", pos)

    plot_cost_history(cost_history=optimizer.cost_history)
    plt.title("Cost history")
    plt.show()

    # TODO: implement grid search pyswarms.utils.search.random_search module

    return


if __name__ == "__main__":
    main()