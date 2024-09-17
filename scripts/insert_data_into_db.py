import json
import mysql.connector
import sys

mydb = mysql.connector.connect(
    host="localhost",
    user="Hannah",
    password="Hannahpwd",
    database="HannahTTLSF"
)

def insert_verbs():
    datas = {}
    with open("data/Verbe.json", "r") as f:
        datas = json.load(f)
    cursor = mydb.cursor()
    for data in datas["words"]:
        print("data", data)
        sql = f"INSERT INTO Verbs (value, tense) VALUES ('{data}', 'infinitive')"
        print("sql", sql)
        try:
            cursor.execute(sql)
        except Exception as e:
            print("error", e, file=sys.stderr)
            continue
    mydb.commit()

def insert_common_nouns():
    datas = {}
    with open("data/Nom commun.json", "r") as f:
        datas = json.load(f)
    cursor = mydb.cursor()
    for data in datas["words"]:
        print("data", data)
        sql = f"INSERT INTO Nouns (value, type) VALUES ('{data}', 'common')"
        print("sql", sql)
        try:
            cursor.execute(sql)
        except Exception as e:
            print("error", e, file=sys.stderr)
            continue
    mydb.commit()

def insert_proper_nouns():
    datas = {}
    with open("data/Nom propre.json", "r") as f:
        datas = json.load(f)
    cursor = mydb.cursor()
    for data in datas["words"]:
        print("data", data)
        sql = f"INSERT INTO Nouns (value, type) VALUES ('{data}', 'proper')"
        print("sql", sql)
        try:
            cursor.execute(sql)
        except Exception as e:
            print("error", e, file=sys.stderr)
            continue
    mydb.commit()

def insert_existing_verbs():
    datas = {}
    with open("../config/verbs.json", "r") as f:
        datas = json.load(f)
    cursor = mydb.cursor()
    for data in datas["conjugations"]:
        print("data", data)
        verb = datas["conjugations"][data]["verb"]
        tense = datas["conjugations"][data]["tense"]
        sql = f"INSERT INTO Verbs (value, tense) VALUES ('{data}', '{tense}')"
        print("sql", sql)
        try:
            cursor.execute(sql)
        except Exception as e:
            print("error", e, file=sys.stderr)
            continue
    mydb.commit()

def insert_existing_determinants():
    datas = {}
    with open("../config/determinants.json", "r") as f:
        datas = json.load(f)
    cursor = mydb.cursor()
    for cat in datas.keys():
        for v in datas[cat]:
            sql = f"INSERT INTO Determinants (value) VALUES ('{v}')"
            print("sql", sql)
            try:
                cursor.execute(sql)
            except Exception as e:
                print("error", e, file=sys.stderr)
                continue
    mydb.commit()

def insert_adjectives():
    datas = {}
    with open("data/Adjectif.json", "r") as f:
        datas = json.load(f)
    cursor = mydb.cursor()
    for data in datas["words"]:
        print("data", data)
        sql = f"INSERT INTO Adjectives (value) VALUES ('{data}')"
        print("sql", sql)
        try:
            cursor.execute(sql)
        except Exception as e:
            print("error", e, file=sys.stderr)
            continue
    mydb.commit()

def insert_adverbes():
    datas = {}
    with open("data/Adverbe.json", "r") as f:
        datas = json.load(f)
    cursor = mydb.cursor()
    for data in datas["words"]:
        print("data", data)
        sql = f"INSERT INTO Adverbes (value) VALUES ('{data}')"
        print("sql", sql)
        try:
            cursor.execute(sql)
        except Exception as e:
            print("error", e, file=sys.stderr)
            continue
    mydb.commit()

def main():
    insert_verbs()
    insert_common_nouns()
    insert_proper_nouns()
    insert_existing_verbs()
    insert_existing_determinants()
    insert_adjectives()
    insert_adverbes()

if __name__ == "__main__":
    main()
