-- CREATE USER admin WITH PASSWORD 'test1';
-- GRANT ALL PRIVILEGES ON DATABASE "my_books" to admin;
CREATE TABLE books (
	id integer PRIMARY KEY UNIQUE NOT NULL,
	author varchar(255) NOT NULL,
	title varchar(255) NOT NULL,
	published_in integer NOT NULL

);