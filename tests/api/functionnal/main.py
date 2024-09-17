import json
import requests

DATA_FILE = "datas.json"
API_URL = "http://localhost:8080"

def call_api(phr):
    url = API_URL + "/api/v1/translate"
    body = {
        "text": phr
    }
    headers = {
        "Content-Type": "application/json"
    }
    r = requests.post(url, json=body, headers=headers)
    return r.text

def main():
    data = {}
    with open(DATA_FILE, "r") as f:
        data = json.load(f)

    for (phr, expect) in data.items():
        res = call_api(phr)
        print(f"Test: {phr}, expect: {expect}, got: {res}")
        assert(res == expect)

if __name__ == "__main__":
    main()
