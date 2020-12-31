import os
import json
from pathlib import Path
from pymongo import MongoClient, errors

# MongoDB
HOST = 'localhost'
PORT = 27017
USER = 'admin'
PASS = 'password123'

# SRD Data
SRD_PATH = Path(__file__).parent / 'srd-data' / 'json'


class MongoSRD:
    _timeout_ms = 3000 

    def __init__(self, 
        host=HOST, 
        port=PORT, 
        user=USER,
        pswd=PASS
    ):
        self.host = host
        self.port = port
        self.user = user
        self.pswd = PASS
        self.client = self.connect()
        self.db = self.client['srd']
        

    def connect(self):
        try:
            client = MongoClient(
                host = f'{self.host}:{self.port}',
                serverSelectionTimeoutMS = self._timeout_ms,
                username = self.user,
                password = self.pswd,
            )
            print(f"Connected to Mongodb Server: {client.server_info()['version']}")
            return client
        except errors.ServerSelectionTimeoutError as err:
            raise err


    def add_source(self, source):
        for obj in source.data:
            self.db[source.header].insert_one(obj.__dict__)



class DataSource:
    def __init__(self, path=SRD_PATH):
        self.path = path
        self.filename = ''
        self.header = ''
        self.data = []

    def crawl_dir(self):
        for dirname, dirnames, filenames in os.walk(self.path / self.form):
            for file in filenames:
                self.load_data(Path(dirname) / file)

    def upload(self, db):
        db.add_source(self)

    def read_file(self, file):
        with open(file, 'r') as f:
            data = json.load(f)
        return data



class Races(DataSource):
    def __init__(self, **kwargs):
        super().__init__()
        self.filename = '01 races.json'
        self.header = 'Races'
        self.load_data()


    def load_data(self, file=None):
        if not file:
            file = self.path / self.filename
        data = self.read_file(file)
        races = data[self.header]
        trait_key = races.pop('Racial Traits') # trait descriptions
        for k,v in races.items():
            self.data.append(Race({k:v}))

    

class Document:
    def __init__(self, data: dict):
        self.name = ''



class Race(Document):
    def __init__(self, *args):
        super().__init__(*args)
        self.traits = {}
        self.subs = {}
        self.parse_data(data)


    def parse_data(self, data: dict):
        self.name = list(data.keys())[0]
        traits = data[self.name][f'{self.name} Traits']
        self.traits = self.parse_traits(traits['content'])
        subs = {k:v for k,v in traits.items() if k != 'content'}
        for k,v in subs.items():
            sub_data = self.parse_traits(v['content'])
            self.subs.update({k:sub_data})


    def parse_traits(self, traits):
        data = {}
        for line in traits:
            if '***' in line:
                parts = line.split('***')
                data.update({parts[1][:-1]:parts[2][1:]})
            else:
                data.update({'intro':line})
        return data



class Classes(DataSource):
    def __init__(self, **kwargs):
        super().__init__()
        self.filename = '02 classes.json'
        self.header = 'Classes'
        self.load_data()


    def load_data(self, file=None):
        if not file:
            file = self.path / self.filename
        data = self.read_file(file)
        classes = data
        for k,v in classes.items():
            self.data.append(Class({k:v}))



class Class:
    def __init__(self, data: dict):
        self.name = ''
        self.traits = {}
        self.subs = {}
        self.parse_data(data)


    def parse_data(self, data: dict):
        self.name = list(data.keys())[0]
        traits = data[self.name][f'{self.name} Traits']
        self.traits = self.parse_traits(traits['content'])
        subs = {k:v for k,v in traits.items() if k != 'content'}
        for k,v in subs.items():
            sub_data = self.parse_traits(v['content'])
            self.subs.update({k:sub_data})



if __name__ == '__main__':
    db = MongoSRD()
    #Races().upload(db)
    Classes().upload(db)