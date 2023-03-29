# 
# FileName: main.py
# FileType: Python file
# Author: Courtney Mueller 
# Date Created: 3 / 27 / 2023
# Description: This program sets up a connection to our mongoDB and allows entries to be added 
#


from pymongo import MongoClient

cluster = "mongodb+srv://admin:OpenCyberCenter@opencybercenter.hxek1qk.mongodb.net/?retryWrites=true&w=majority"
client = MongoClient(cluster)

print(client.list_database_names())
db = client.test

print(db.list_collection_names())

# add data into database
# can add multiple entries at once using dictonary style list
entryList = [
  {"_id": 0, "name": "Bob", "username": "bobusername","password": "bobpassword"},
  {"_id": 1, "name": "Charlie", "username": "charlieusername", "password": "charliepassword"}
]

# can also add in a singluar entry 
singleEntry = {"_id": 2, 
               "name": "Dan", 
               "username": "danusername",
               "password": "danpassword"
              }

# add above information into passwordsTest database 
# using add multiple method 
passwords = db.passwordsTest
passwords = passwords.insert_many(entryList)

# using singular entry addition method 
passwords = passwords.insert_one(singleEntry)
