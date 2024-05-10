CREATE TABLE tags(
	tag_id int unsigned auto_increment primary key,
	tag_name varchar(64) not null,
	tag_description varchar(512) not null,
	tag_hash varchar(256) not null unique
);
CREATE TABLE authors(
	author_id int unsigned auto_increment primary key,
	author_name varchar(128) not null,
	author_email varchar(256) not null,
	author_bio varchar(1024) not null,
	author_intro varchar(256) not null,
	author_photo_url varchar(128) not null,
	author_hash varchar(256) not null unique
) auto_increment = 34556;
CREATE TABLE posts(
	post_id int unsigned auto_increment primary key,
	post_url varchar(256) not null,
	title varchar(256) not null,
	body text not null,
	post_description varchar(512) not null,
	tldr varchar(1024) not null,
	subtitle varchar(256) not null,
	published timestamp not null,
	hero_image varchar(256) not null,
	post_hash varchar(256) not null unique
) auto_increment = 12340;
CREATE TABLE post_keywords(
	keyword_id int unsigned auto_increment primary key,
	post_id int unsigned,
	keyword varchar(64) not null,
	keyword_hash varchar(256) not null unique
) auto_increment = 66789;
CREATE TABLE posts_tags (
    tag_id INT UNSIGNED,
    post_id INT UNSIGNED,
	row_hash varchar(256) not null unique,
    FOREIGN KEY (tag_id) REFERENCES tags(tag_id),
    FOREIGN KEY (post_id) REFERENCES posts(post_id)
);
CREATE TABLE posts_authors (
	author_id INT UNSIGNED,
	post_id INT UNSIGNED,
	row_hash varchar(256) not null unique,
	FOREIGN KEY (author_id) REFERENCES authors(author_id),
	FOREIGN KEY (post_id) REFERENCES posts(post_id)
);