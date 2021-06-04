CREATE TABLE allows (
  id INTEGER NOT NULL PRIMARY KEY,
  pid_endpoint INTEGER NOT NULL,
  pid_content INTEGER NOT NULL,
  status_code INTEGER NOT NULL,
  request_method TEXT CHECK(request_method IN ('GET','POST','PUT', 'DELETE')) NOT NULL,
  flag INTEGER NOT NULL DEFAULT '1',
  UNIQUE(pid_endpoint, request_method)
)