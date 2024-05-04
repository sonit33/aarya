use std::{collections::HashMap, path::Path};

use aarya_utils::file_ops::{read_file_contents, FileOpsResult};
use models::{
    blogs::{PostAuthorEntity, PostEntity, PostManifestModel, PostTagEntity, TagEntity},
    result_types::EntityResult,
};
use sqlx::MySqlPool;

pub async fn run_blog_poster(
    manifest_file: &Path,
    pool: &MySqlPool,
) {
    // load the manifest
    let manifest = match read_file_contents(manifest_file.to_str().unwrap()) {
        FileOpsResult::Success(contents) => match serde_json::from_str::<PostManifestModel>(contents.as_str()) {
            Ok(m) => m,
            Err(e) => {
                println!("Failed to parse manifest file: {:?}", e);
                return;
            }
        },
        FileOpsResult::Error(e) => {
            println!("Failed to read manifest file: {:?}", e);
            return;
        }
    };

    // retrieve the post content
    let post_body = match read_file_contents(&manifest.post_file) {
        FileOpsResult::Success(body) => body,
        FileOpsResult::Error(e) => {
            println!("Failed to read post file: {:?}", e);
            return;
        }
    };

    // retrieve the post tags
    let mut tags = HashMap::new();
    for tag in manifest.tags {
        let mut tag_entity = TagEntity::new();
        tag_entity.tag_id = tag;
        match tag_entity.find_tag(pool).await {
            EntityResult::Success(t) => {
                tags.insert(t.tag_id, t.tag_name);
            }
            EntityResult::Error(e) => {
                println!("Failed to retrieve tag: {:?}", e);
                return;
            }
        }
    }

    // form the partial url path of the post from tags and title
    let mut post_url: Vec<String> = Vec::new();
    tags.clone().into_iter().for_each(|(_id, name)| {
        // lowercase name and spaces replaced with hyphens
        post_url.push(name.to_lowercase().replace(' ', "-"));
    });
    post_url.push(manifest.post_title.to_lowercase().replace(' ', "-"));
    let post_url = post_url.join("/");

    println!("Post URL: {}", post_url);

    // save the post to the database
    let post = PostEntity {
        post_url,
        post_title: manifest.post_title,
        post_body,
        post_description: manifest.description,
        post_keywords: manifest.keywords.join(", "),
        post_summary: manifest.tldr,
        post_timestamp: manifest.display_date,
        post_id: None,
        post_hash: None,
    };

    let post_id = match post.create(pool).await {
        EntityResult::Success(p) => match p {
            models::result_types::SuccessResultType::Created(id, _) => id as u32,
            _ => {
                println!("Failed to create post: {:?}", p);
                return;
            }
        },
        EntityResult::Error(e) => {
            println!("Failed to create post: {:?}", e);
            return;
        }
    };

    // use post_id and the tags to save the post tags
    for tag in tags {
        let post_tag = PostTagEntity {
            post_id,
            tag_id: tag.0,
            row_hash: None,
        };

        match post_tag.create(pool).await {
            EntityResult::Success(_) => (),
            EntityResult::Error(e) => {
                println!("Failed to save post tags: {:?}", e);
                return;
            }
        }
    }

    // save posts authors
    for author in manifest.authors {
        let post_author = PostAuthorEntity {
            post_id,
            author_id: author,
            row_hash: None,
        };
        match post_author.create(pool).await {
            EntityResult::Success(_) => (),
            EntityResult::Error(e) => {
                println!("Failed to save post authors: {:?}", e);
                return;
            }
        }
    }
}
