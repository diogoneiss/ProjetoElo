import os
from pymongo import MongoClient
import pandas as pd
from dotenv import load_dotenv
import pathlib
import uuid

# get the path of the current directory
current_dir = pathlib.Path(__file__).parent.absolute()

# construct the path to the .env file in the parent directory (elo_compnat)
dotenv_path = current_dir.parent / '.env'

ENV_EXISTS = True
try:

    # check if the .env file exists
    if not os.path.exists(dotenv_path):
        raise FileNotFoundError

    # Load variables from .env
    load_dotenv(dotenv_path)

    MONGODB_URI = os.getenv('MONGODB_URI')
    DATABASE_NAME = os.getenv('DATABASE_NAME')
    COLLECTION_NAME = os.getenv('COLLECTION_NAME')

    # create a client connection to your MongoDB instance
    client = MongoClient(MONGODB_URI)

    # connect to your database
    db = client[DATABASE_NAME]

    # connect to your collection
    collection = db[COLLECTION_NAME]
except FileNotFoundError:
    ENV_EXISTS = False
    horizontal_line = "--------------------------------------------------" * 2

    print("\n\n\n")
    print(horizontal_line)
    print("The .env file does not exist. Please create one in the parent directory to use the remote db.")
    print(horizontal_line)
    print("\n\n\n")
    

def insert_document(df=pd.DataFrame({'A': [1, 2, 3], 'B': [4, 5, 6]})):
    if ENV_EXISTS:
        # transform the DataFrame to a nested dictionary and convert pandas series to list
        data = df.to_dict(orient='series')
        data = {key: value.tolist() for key, value in data.items()}

        # todo: usar os parametros pra criar um hash, que sera o uuid
        data['_id'] = str(uuid.uuid4())
        collection.insert_one(data)
    else:
        print("Cannot insert document because .env file does not exist.")

def find_by_id(id):
    pass

def find_by_parameters(parameters):
    pass
