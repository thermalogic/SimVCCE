# The Simple Web Server with Flask

* ./webservice/

## Prerequisites

* Server: simvcce, flask
* Client: requests

## Codes

* The Server: `app.py`
  
* The Client: `client.py`

* The Cycle Data: `./jsonmodel/*.json`
 
## The Web Service

**Local Web Service**

In the terminal of ./webservice/
```bash
python appt.py
```
* https://127.0.0.1:5000/simvcce
 
**Remote Web Service** 

* https://simvcce.herokuapp.com/simvcce 

## Using the Web Service

In the terminal of ./webservice/

```bash
python client.py
```

or 

```bash
$curl  https://simvcce.herokuapp.com/simvcce POST -d @./jsonmodel/ivcr_11_1.json --header "Content-Type: application/json"
```

**Note:** The URL in the above is Remote Web Service.

## References

**Flask**

* [Flask Quickstart](https://flask.palletsprojects.com/en/2.1.x/quickstart/)

* [Flask Tutorial](https://flask.palletsprojects.com/en/2.1.x/tutorial/)

* [VS Code:Tutorial Flask](https://code.visualstudio.com/docs/python/tutorial-flask)

* [NTU: Python WebApp: Flask](https://www3.ntu.edu.sg/home/ehchua/programming/webprogramming/Python3_Flask.html)

**Requests**

* https://requests.readthedocs.io/en/latest/
