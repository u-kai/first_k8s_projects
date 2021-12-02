DROP SCHEMA IF EXISTS sample;
CREATE SCHEMA sample;
USE sample;

DROP TABLE IF EXISTS users_login;

CREATE TABLE users_login
(
    user_id  INT,
    name     VARCHAR(40),
    password VARCHAR(256)
);

INSERT INTO users_login (user_id,name,password) VALUES (1,"kai","kaipassword");
INSERT INTO users_login (user_id,name,password) VALUES (2,"minami","minamipassword");
INSERT INTO users_login (user_id,name,password) VALUES (3,"mio","miopassword");

