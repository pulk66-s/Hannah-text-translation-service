import bs4
import requests
import json

URL = "https://www.le-dictionnaire.com/repertoire/"

def get_words(letter, page_nb):
    url = f"{URL}{letter}{page_nb:02d}"
    page_bs = bs4.BeautifulSoup(requests.get(url).text, "html.parser")
    alphabox = page_bs.find_all("div", {"class": "alphabox"})[0]
    ul = alphabox.find_all("ul")[0]
    li = ul.find_all("li")
    return [l.text for l in li]

def main():
    all_words = []
    for letter in "abcdefghijklmnopqrstuvwxyz":
        print("letter", letter)
        main_page = bs4.BeautifulSoup(requests.get(URL + letter + "01.html").text, "html.parser")
        sub_divs = main_page.find_all("div", {"id": "navalpha"})[0]
        sub_letters_list = sub_divs.find_all("span")[0].find_all("a")
        nb_sub_letters = len(sub_letters_list)
        for page_nb in range(1, nb_sub_letters + 1):
            print("page_nb", page_nb)
            all_words += get_words(letter, page_nb)
    data = {
        "words": all_words
    }
    with open("data.json", "w") as f:
        json.dump(data, f)

if __name__ == "__main__":
    main()
