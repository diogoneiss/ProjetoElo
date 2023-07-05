# Como rodar inicialmente

1. Criar venv do python, a biblioteca recomenda o mkvirtualenv para criar/sair deles (gostei bastante, mas tem que configurar seu `.bashrc`)
2. Entrar no seu venv
3. `pip install maturin` e as demais dependências

## Como compilar para rodar o python

1. `maturin develop`, dentro da elo_compnat

## Como rodar o python

Só chamar `python python/main.py`

# TODO
1. Fazer a função de backtesting registrar a frequência e outros parâmetros, igual fazemos na experimentação
2. Parse dos dados
3. Incluir coluna de home e away elo nos csvs
5. Função que monta o dataset para regressão (talvez só salvar num path e ler...)
6. Salvar elo nas partidas