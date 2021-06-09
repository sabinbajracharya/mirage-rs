CREATE TABLE contents (
  id INTEGER NOT NULL PRIMARY KEY,
  pid_endpoint INTEGER NOT NULL,
  body TEXT NOT NULL,
  status_code INTEGER NOT NULL,
  request_method TEXT CHECK( request_method IN ('GET','POST','PUT', 'DELETE')) NOT NULL,
  FOREIGN KEY (pid_endpoint) REFERENCES endpoints(id)
)