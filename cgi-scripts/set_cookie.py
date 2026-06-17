#!/usr/bin/env python3
import os

session_id = os.environ.get("SESSION_ID", "0")

print("Status: 200 OK")
print("Content-Type: text/html")
print(f"Set-Cookie: session_id={session_id}; Path=/admin; HttpOnly")
print("")
# add href to /admin in the html response
print("<html><body><h1>Cookie was set.</h1><p>Go back to <a href='/admin'>/admin</a>.</p></body></html>")
