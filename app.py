import webbrowser
import time

urls = [] # TODO: read from file

browser = webbrowser.get('firefox') # TODO: allow browser choice

for url in urls:
    browser.open_new_tab(url)
    time.sleep(1)
