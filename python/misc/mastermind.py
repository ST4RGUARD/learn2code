import requests, json

email = 'jmjsuperman@hotmail.com'

r = requests.post('https://mastermind.praetorian.com/api-auth-token/', data={'email':email})
r.json()

# > {'Auth-Token': 'AUTH_TOKEN'}
headers = r.json()
print headers

# Interacting with the game
r = requests.get('https://mastermind.praetorian.com/level/1/', headers=headers)
r.json()


# pseudo
wpns  = [1,2,3,4,5,6]
guess = [1,2,3,4]

# > {'numGladiators': 4, 'numGuesses': 8, 'numRounds': 1, 'numWeapons': 6}
r = requests.post('http://mastermind.praetorian.com/level/1/', data=json.dumps({'guess':[6,5,1,3]}), headers=headers)
print r.json()
# > {'response': [2, 1]}

