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
    username NAME PRIMARY KEY NOT NULL UNIQUE,
    password VARCHAR(24) NOT NULL,
    last_name NAME NOT NULL,
    first_name NAME NOT NULL,
    user_specs user_specs NOT NULL,
    email VARCHAR(255),
    phone_number VARCHAR(13)
);

CREATE TABLE students(
    username NAME NOT NULL UNIQUE REFERENCES users(username),
    class_num SMALLINT NOT NULL,
    class_char VARCHAR(1) NOT NULL,
    school_num SMALLINT NOT NULL                   
);

CREATE TABLE teachers(
    username NAME NOT NULL UNIQUE REFERENCES users(username),
    school_num SMALLINT NOT NULL,
    subject subject  NOT NULL
);

CREATE TABLE administrators(
    username NAME NOT NULL UNIQUE REFERENCES users(username),
    job_title TEXT NOT NULL,
    school_num SMALLINT NOT NULL    
);

CREATE TABLE posts(
    uuid UUID PRIMARY KEY NOT NULL,
    title_on_russian TEXT NOT NULL,
    title_on_uzbek TEXT NOT NULL,
    title_on_english TEXT NOT NULL,
    content_on_russian TEXT NOT NULL,
    content_on_uzbek TEXT NOT NULL,
    content_on_english TEXT NOT NULL,
    published_at TimeStamp NOT NULL DEFAULT NOW(),
    edited BOOLEAN NOT NULL,
    edited_at TimeStamp,
    written_by NAME NOT NULL REFERENCES users(username),
    written_on VARCHAR(2),
    likes BIGINT NOT NULL,
    dislikes BIGINT NOT NULL
);

CREATE TABLE comments(
    uuid UUID PRIMARY KEY NOT NULL,
    content_on_russian TEXT NOT NULL,
    content_on_uzbek TEXT NOT NULL,
    content_on_english TEXT NOT NULL,
    published_at TimeStamp NOT NULL DEFAULT NOW(),
    edited BOOLEAN NOT NULL,
    edited_at TimeStamp,
    written_by NAME NOT NULL REFERENCES users(username),
    written_on VARCHAR(2),
    likes BIGINT NOT NULL,
    dislikes BIGINT NOT NULL,
    replys_for UUID REFERENCES comments(uuid)
);

