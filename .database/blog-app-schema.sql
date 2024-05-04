CREATE TABLE tags(
	tag_id int unsigned auto_increment primary key,
	tag_name varchar(64) not null,
	tag_description varchar(512) not null
);
CREATE TABLE authors(
	author_id int unsigned auto_increment primary key,
	author_name varchar(128) not null,
	author_email varchar(255) not null,
	author_bio varchar(1024) not null,
	author_photo_url varchar(128) not null
) auto_increment = 34556;
CREATE TABLE posts(
	post_id int unsigned auto_increment primary key,
	post_url varchar(512) not null,
	post_title varchar(255) not null,
	post_body text not null,
	post_description varchar(512) not null,
	post_keywords varchar(255) not null,
	post_summary varchar(1024) not null,
	post_timestamp timestamp not null,
	author_id int unsigned not null
) auto_increment = 12340;
CREATE TABLE posts_tags (
    tag_id INT UNSIGNED,
    post_id INT UNSIGNED,
    FOREIGN KEY (tag_id) REFERENCES tags(tag_id),
    FOREIGN KEY (post_id) REFERENCES posts(post_id)
);