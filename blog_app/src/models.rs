use std::vec;

use log::error;
use mongodb::{bson::oid::ObjectId, Collection};
use serde::{Deserialize, Serialize};

use crate::{
    entities::{
        blogs::{AuthorEntity, PostEntity, TagEntity},
        result_types::EntityResult,
    },
    utils::{date_ops, db_ops::Database, image_ops::ImagePath},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthorRequestModel {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub bio: String,
    pub photo_url: String,
    pub intro: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthorResponseModel {
    pub id: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub bio: String,
    pub profile_photo: String,
    pub thumbnail_photo: String,
    pub intro: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TagRequestModel {
    pub name: String,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TagResponseModel {
    pub id: String,
    pub name: String,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostResponseModel {
    pub id: String,
    pub permalink: String,
    pub title: String,
    pub subtitle: String,
    pub keywords: String,
    pub body: String,
    pub description: String,
    pub tldr: String,
    pub hero_image: String,
    pub profile_image: String,
    pub publish_date: String,
    pub modified_date: String,
    pub author: AuthorResponseModel,
    pub tag: TagResponseModel,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostRequestModel {
    pub permalink: String,
    pub title: String,
    pub subtitle: String,
    pub keywords: String,
    pub body: String,
    pub description: String,
    pub tldr: String,
    pub hero_image: String,
    pub publish_date: i64,
    pub modified_date: i64,
    pub author: String,
    pub tag: String,
}

impl AuthorRequestModel {
    pub fn to(&self) -> AuthorEntity {
        AuthorEntity {
            _id: None,
            first_name: self.first_name.clone(),
            email: self.email.to_string(),
            bio: self.bio.to_string(),
            photo_url: ImagePath::from_string(self.photo_url.as_str()),
            intro: self.intro.to_string(),
            last_name: self.last_name.clone(),
        }
    }
}

impl Default for PostResponseModel {
    fn default() -> Self {
        PostResponseModel {
            id: "not-set".to_string(),
            permalink: "not-set".to_string(),
            title: "not-set".to_string(),
            keywords: "not-set".to_string(),
            subtitle: "not-set".to_string(),
            body: "not-set".to_string(),
            description: "not-set".to_string(),
            tldr: "not-set".to_string(),
            hero_image: "not-set".to_string(),
            profile_image: "not-set".to_string(),
            publish_date: date_ops::local_date().to_string(),
            modified_date: date_ops::local_date().to_string(),
            author: AuthorResponseModel::default(),
            tag: TagResponseModel::default(),
        }
    }
}

impl Default for AuthorResponseModel {
    fn default() -> Self {
        AuthorResponseModel {
            id: "not-set".to_string(),
            first_name: "not-set".to_string(),
            last_name: "not-set".to_string(),
            email: "not-set".to_string(),
            bio: "not-set".to_string(),
            profile_photo: "not-set".to_string(),
            thumbnail_photo: "not-set".to_string(),
            intro: "not-set".to_string(),
        }
    }
}

impl AuthorResponseModel {
    pub fn from(entity: AuthorEntity) -> Self {
        AuthorResponseModel {
            id: entity._id.unwrap().to_string(),
            first_name: entity.first_name,
            last_name: entity.last_name,
            email: entity.email,
            bio: entity.bio,
            profile_photo: entity.photo_url.profile_r_path(),
            thumbnail_photo: entity.photo_url.thumbnail_r_path(),
            intro: entity.intro,
        }
    }

    pub fn from_vec(entities: Vec<AuthorEntity>) -> Vec<Self> {
        let mut authors = vec![];

        for entity in entities {
            authors.push(AuthorResponseModel::from(entity));
        }

        authors
    }
}

impl TagRequestModel {
    pub fn to(&self) -> TagEntity {
        TagEntity {
            _id: None,
            name: self.name.to_string(),
            description: self.description.to_string(),
        }
    }
}

impl Default for TagResponseModel {
    fn default() -> Self {
        TagResponseModel {
            id: "not-set".to_string(),
            name: "not-set".to_string(),
            description: "not-set".to_string(),
        }
    }
}

impl TagResponseModel {
    pub fn from(entity: TagEntity) -> Self {
        TagResponseModel {
            id: entity._id.unwrap().to_string(),
            name: entity.name.to_string(),
            description: entity.description.to_string(),
        }
    }

    pub fn from_vec(entities: Vec<TagEntity>) -> Vec<Self> {
        let mut tags = vec![];

        for entity in entities {
            tags.push(TagResponseModel::from(entity));
        }

        tags
    }
}

impl PostResponseModel {
    pub fn from(entity: PostEntity) -> Self {
        PostResponseModel {
            id: entity._id.unwrap().to_string(),
            permalink: entity.permalink.to_string(),
            title: entity.title.to_string(),
            keywords: entity.kicker.to_string(),
            subtitle: entity.subtitle.to_string(),
            body: entity.body.to_string(),
            description: entity.description.to_string(),
            tldr: entity.tldr.to_string(),
            hero_image: entity.hero_image.hero_r_path(),
            profile_image: entity.hero_image.profile_r_path(),
            publish_date: date_ops::to_display_date(entity.publish_date).to_string(),
            modified_date: date_ops::to_display_date(entity.modified_date).to_string(),
            author: AuthorResponseModel::default(),
            tag: TagResponseModel::default(),
        }
    }

    pub fn from_vec(entities: Vec<PostEntity>) -> Vec<Self> {
        let mut posts = vec![];

        for entity in entities {
            posts.push(PostResponseModel::from(entity));
        }

        posts
    }

    pub async fn from_permalink(permalink: String, collection: Collection<PostEntity>) -> Self {
        match Database::find_by(collection, String::from("permalink"), permalink).await {
            EntityResult::Success(r) => PostResponseModel::from(r),
            EntityResult::Error(e) => {
                error!("Failed to find post by permalink: {:?}", e);
                PostResponseModel::default()
            }
        }
    }

    pub fn all(
        posts: Vec<PostEntity>,
        authors: Vec<AuthorEntity>,
        tags: Vec<TagEntity>,
    ) -> Vec<PostResponseModel> {
        let mut post_responses = Vec::new();

        for post in posts {
            let post_author = AuthorResponseModel::from(
                authors
                    .iter()
                    .find(|author| {
                        author
                            ._id
                            .unwrap()
                            .eq(&ObjectId::parse_str(post.author.as_str()).unwrap())
                    })
                    .unwrap()
                    .clone(),
            );

            let post_tag = TagResponseModel::from(
                tags.iter()
                    .find(|tag| {
                        tag._id
                            .unwrap()
                            .eq(&ObjectId::parse_str(post.tag.as_str()).unwrap())
                    })
                    .unwrap()
                    .clone(),
            );

            let post_response = PostResponseModel {
                id: post._id.unwrap().to_string(),
                permalink: post.permalink.clone(),
                title: post.title.clone(),
                keywords: post.kicker.clone(),
                subtitle: post.subtitle.clone(),
                body: post.body.clone(),
                description: post.description.clone(),
                tldr: post.tldr.clone(),
                hero_image: post.hero_image.hero_r_path(),
                profile_image: post.hero_image.profile_r_path(),
                publish_date: date_ops::to_display_date(post.publish_date).to_string(),
                modified_date: date_ops::to_display_date(post.modified_date).to_string(),
                author: post_author,
                tag: post_tag,
            };

            post_responses.push(post_response);
        }

        post_responses
    }

    pub fn combine(entity: PostEntity, authors: Vec<AuthorEntity>, tags: Vec<TagEntity>) -> Self {
        PostResponseModel {
            id: entity._id.unwrap().to_string(),
            permalink: entity.permalink.to_string(),
            title: entity.title.to_string(),
            keywords: entity.kicker.to_string(),
            subtitle: entity.subtitle.to_string(),
            body: entity.body.to_string(),
            description: entity.description.to_string(),
            tldr: entity.tldr.to_string(),
            hero_image: entity.hero_image.hero_r_path(),
            profile_image: entity.hero_image.profile_r_path(),
            publish_date: date_ops::to_display_date(entity.publish_date).to_string(),
            modified_date: date_ops::to_display_date(entity.modified_date).to_string(),
            author: authors
                .iter()
                .find(|author| {
                    author
                        ._id
                        .unwrap()
                        .eq(&ObjectId::parse_str(entity.author.as_str()).unwrap())
                })
                .map(|author| AuthorResponseModel::from(author.clone()))
                .unwrap(),
            tag: tags
                .iter()
                .find(|tag| {
                    tag._id
                        .unwrap()
                        .eq(&ObjectId::parse_str(entity.tag.as_str()).unwrap())
                })
                .map(|tag| TagResponseModel::from(tag.clone()))
                .unwrap(),
        }
    }
}

impl PostRequestModel {
    pub fn to(&self) -> PostEntity {
        PostEntity {
            permalink: self.permalink.to_string(),
            title: self.title.to_string(),
            body: self.body.to_string(),
            description: self.description.to_string(),
            tldr: self.tldr.to_string(),
            hero_image: ImagePath::from_string(self.hero_image.as_str()),
            subtitle: self.subtitle.to_string(),
            _id: None,
            author: self.author.clone(),
            tag: self.tag.clone(),
            kicker: self.keywords.to_string(),
            publish_date: date_ops::from(self.publish_date),
            modified_date: date_ops::from(self.modified_date),
        }
    }
}
