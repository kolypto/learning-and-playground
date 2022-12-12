from flask import Flask
from werkzeug import run_simple

app = Flask(__name__)


@app.route('/')
def index():
    return 'I respond; consequently, I exist'


@app.route('/ping')
def ping():
    return 'Pong'


if __name__ == '__main__':
    run_simple(
        application=app,
        hostname='0.0.0.0',
        port=5000,
        use_reloader=True,
        use_debugger=True,
    )
