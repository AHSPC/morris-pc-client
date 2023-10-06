import requests
import schedule
import time

def getLatest():
    result = requests.get('http://google.com')
    if result.status_code == 200:
        print(result.json())

schedule.every().minute.at(':00').do(getLatest)

while True:
    time.sleep(1)
    schedule.run_pending()
