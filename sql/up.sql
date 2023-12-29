CREATE TYPE user_specs AS ENUM (
  'Teacher',
  'Student',
  'Administrator',
  'Other'
);

CREATE TYPE subject AS ENUM (
  'Mathematics',
  'Physics',
  'Chemistry',
  'Biology',
  'Uzbek',
  'Russian',
  'English',
  'History',
  'Geography',
  'Literature',
  'Physical Education',
  'Computer Science',
  'Economics',
  'Law',
  'Education'
);

CREATE TABLE users(
    uuid UUID PRIMARY KEY NOT NULL UNIQUE,
    username NAME NOT NULL UNIQUE,
    about VARCHAR(500) NOT NULL,
    password VARCHAR(24) NOT NULL,
    last_name NAME NOT NULL,
    first_name NAME NOT NULL,
    user_specs user_specs NOT NULL,
    email VARCHAR(255) NOT NULL,
    birth_date DATE NOT NULL,
    phone_number VARCHAR(13)
);

CREATE TABLE students(
    username NAME NOT NULL UNIQUE REFERENCES users(username),
    class_num SMALLINT NOT NULL,
    class_char VARCHAR(1) NOT NULL
);

CREATE TABLE teachers(
    username NAME NOT NULL UNIQUE REFERENCES users(username),
    subject subject  NOT NULL
);

CREATE TABLE administrators(
    username NAME NOT NULL UNIQUE REFERENCES users(username),
    job_title TEXT NOT NULL
);

CREATE TABLE posts(
    uuid UUID PRIMARY KEY NOT NULL,
    title TEXT NOT NULL,
    content TEXT NOT NULL,
    published_at TimeStamp NOT NULL DEFAULT NOW(),
    edited BOOLEAN NOT NULL,
    edited_at TimeStamp,
    written_by NAME NOT NULL REFERENCES users(username),
    tags TEXT[] NOT NULL
);

CREATE TABLE post_mark(
    uuid UUID PRIMARY KEY NOT NULL,
    username TEXT NOT NULL REFERENCES users(username),
    post UUID NOT NULL REFERENCES posts(uuid),
    liked BOOLEAN NOT NULL
);


CREATE TABLE comment_mark(
    uuid UUID PRIMARY KEY NOT NULL,
    username TEXT NOT NULL REFERENCES users(username),
    post UUID NOT NULL REFERENCES comments(uuid),
    liked BOOLEAN NOT NULL
);

CREATE TABLE comments(
    uuid UUID PRIMARY KEY NOT NULL,
    written_under UUID NOT NULL REFERENCES posts(uuid),
    content TEXT NOT NULL,
    published_at TimeStamp NOT NULL DEFAULT NOW(),
    edited BOOLEAN NOT NULL DEFAULT FALSE,
    edited_at TimeStamp DEFAULT NULL,
    written_by NAME NOT NULL REFERENCES users(username),
    replys_for UUID REFERENCES comments(uuid)
);

