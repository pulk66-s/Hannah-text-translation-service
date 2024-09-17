import json
import requests
import bs4
import threading

base_url = "https://www.le-dictionnaire.com/definition/"
infos = {}

def pass_cond(word):
    if word.count(" ") > 0:
        return False
    return True

def filter_words(words):
    return [w for w in words if pass_cond(w)]

def call_site(word, i, nb_words):
    print("word:", word, "i:", i, "nb_words:", nb_words)
    page_bs = bs4.BeautifulSoup(requests.get(base_url + word).text, "html.parser")
    defbox = page_bs.find("div", {"class": "defbox"})
    span = defbox.find("span")
    category = span.text.split("(")[1].split(")")[0]
    infos[word] = category

def get_words_infos(words):
    nb_max_launch_at_a_time = 100
    nb_words = len(words)
    for i in range(0, nb_words, nb_max_launch_at_a_time):
        threads = []
        for j in range(nb_max_launch_at_a_time):
            if i + j >= nb_words:
                break
            word = words[i + j]
            thread = threading.Thread(target=call_site, args=(word, i + j, nb_words))
            thread.start()
            threads.append(thread)
        for thread in threads:
            thread.join()

def main():
    data = {}
    with open("data.json", "r") as f:
        data = json.load(f)
    words = data["words"]
    words = filter_words(words)
    get_words_infos(words)
    with open("words_infos.json", "w") as f:
        json.dump(infos, f)

if __name__ == "__main__":
    main()
