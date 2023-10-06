import requests

def getLatest():
    result = requests.post('http://httpbin.org/post', json={"key": "value"})
    if result.status_code == 200:
        return result.test

print getLatest()
