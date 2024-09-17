import json
import sys
import os

def get_all_words():
    words = {}
    with open("./words_infos.json", "r") as f:
        words = json.load(f)
    return words

def get_json_files():
    files = {}
    with open("../config/adjectives.json", "r") as f:
        files["adj"] = json.load(f)
    with open("../config/adverbes.json", "r") as f:
        files["adv"] = json.load(f)
    with open("../config/common_nouns.json", "r") as f:
        files["cn"] = json.load(f)
    with open("../config/coordinating_conjunctions.json", "r") as f:
        files["cc"] = json.load(f)
    with open("../config/demonstrative_pronouns.json", "r") as f:
        files["dp"] = json.load(f)
    with open("../config/determinants.json", "r") as f:
        files["det"] = json.load(f)
    with open("../config/prepositions.json", "r") as f:
        files["prep"] = json.load(f)
    with open("../config/proper_nouns.json", "r") as f:
        files["pn"] = json.load(f)
    with open("../config/relative_pronouns.json", "r") as f:
        files["rp"] = json.load(f)
    with open("../config/subject_pronouns.json", "r") as f:
        files["sp"] = json.load(f)
    with open("../config/verbs.json", "r") as f:
        files["v"] = json.load(f)
    return files

def write_all_files(files):
    print("../config/adjectives.json")
    with open("../config/adjectives.json", "w") as f:
        json_string = json.dumps(files["adj"], indent=4, ensure_ascii=False).encode("utf-8")
        f.write(json_string.decode())
    print("../config/adverbes.json")
    with open("../config/adverbes.json", "w") as f:
        json_string = json.dumps(files["adv"], indent=4, ensure_ascii=False).encode("utf-8")
        f.write(json_string.decode())
    print("../config/common_nouns.json")
    with open("../config/common_nouns.json", "w") as f:
        json_string = json.dumps(files["cn"], indent=4, ensure_ascii=False).encode("utf-8")
        f.write(json_string.decode())
    print("../config/coordinating_conjunctions.json")
    with open("../config/coordinating_conjunctions.json", "w") as f:
        json_string = json.dumps(files["cc"], indent=4, ensure_ascii=False).encode("utf-8")
        f.write(json_string.decode())
    print("../config/demonstrative_pronouns.json")
    with open("../config/demonstrative_pronouns.json", "w") as f:
        json_string = json.dumps(files["dp"], indent=4, ensure_ascii=False).encode("utf-8")
        f.write(json_string.decode())
    print("../config/determinants.json")
    with open("../config/determinants.json", "w") as f:
        json_string = json.dumps(files["det"], indent=4, ensure_ascii=False).encode("utf-8")
        f.write(json_string.decode())
    print("../config/prepositions.json")
    with open("../config/prepositions.json", "w") as f:
        json_string = json.dumps(files["prep"], indent=4, ensure_ascii=False).encode("utf-8")
        f.write(json_string.decode())
    print("../config/proper_nouns.json")
    with open("../config/proper_nouns.json", "w") as f:
        json_string = json.dumps(files["pn"], indent=4, ensure_ascii=False).encode("utf-8")
        f.write(json_string.decode())
    print("../config/relative_pronouns.json")
    with open("../config/relative_pronouns.json", "w") as f:
        json_string = json.dumps(files["rp"], indent=4, ensure_ascii=False).encode("utf-8")
        f.write(json_string.decode())
    print("../config/subject_pronouns.json")
    with open("../config/subject_pronouns.json", "w") as f:
        json_string = json.dumps(files["sp"], indent=4, ensure_ascii=False).encode("utf-8")
        f.write(json_string.decode())
    print("../config/verbs.json")
    with open("../config/verbs.json", "w") as f:
        json_string = json.dumps(files["v"], indent=4, ensure_ascii=False).encode("utf-8")
        f.write(json_string.decode())

def replace_utf8(word):
    word = word.lower()
    word = word.replace("é", "e")
    word = word.replace("è", "e")
    word = word.replace("ê", "e")
    word = word.replace("à", "a")
    word = word.replace("â", "a")
    word = word.replace("î", "i")
    word = word.replace("ï", "i")
    word = word.replace("ô", "o")
    word = word.replace("û", "u")
    word = word.replace("ù", "u")
    return word

def sort_words():
    words = get_all_words()
    # files = get_json_files()
    categories = {}
    for word in words:
        category = words[word]
        if category not in categories:
            categories[category] = []
        categories[category].append(word)
    for cat in categories:
        with open(f"./data/{cat}.json", "w") as f:
            json_data = {
                "words": categories[cat]
            }
            json_string = json.dumps(json_data, indent=4, ensure_ascii=False).encode("utf-8")
            f.write(json_string.decode())
    # write_all_files(files)
    # print(files["adj"]["adjectives"])

def merge_adjectives():
    file_list = os.listdir("./data")
    file_list = [f for f in file_list if "Adjectif" in f]
    words = []
    for f in file_list:
        with open(f"./data/{f}", "r") as f:
            json_data = json.load(f)
            words += json_data["words"]
    print("words", words)

def main():
    merge_adjectives()

if __name__ == "__main__":
    main()
