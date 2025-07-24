# Import the Flask class from the flask module
from flask import Flask

# Create an instance of the Flask class, passing in the name of the current module
app = Flask(__name__)


# Define a route for the root URL ("/")
@app.route("/")
def home():
    return "Hello, World!"
