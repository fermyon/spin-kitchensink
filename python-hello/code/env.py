import cgi

print('Content-Type: text/plain; charset=UTF-8')
print('Status: 200')
print()

print('Hello, from Python!')

params = cgi.parse()
print(params)
