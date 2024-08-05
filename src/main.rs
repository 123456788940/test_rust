use std::collections::HashMap;
use rand::Rng;

#[derive(Debug, Clone)]
struct post {
    id: String,
    content: String,
    comments: Vec<String>,
    likes: u32,
    dislikes: u32,


}

impl post {
    fn new(content: &str) -> Self {
        Self {
            id: generate_unique_id(),
            content: content.to_string(),
            comments: Vec::new(),
            likes: 0,
            dislikes: 0,
        }
    }

    fn add_comment(&mut self, comment: &str) {
        self.comments.push(comment.to_string());
    }

    fn like(&mut self) {
        self.likes += 1;
    }

    
    fn dislike(&mut self) {
        self.dislikes += 1;
    }
}
    fn generate_unique_id() -> String {
        let mut rng = rand::thread_rng();
        let id: u32 = rng.gen_range(1000..9999);
        id.to_string()

    }

    struct socialMediaPlatform {
        posts: HashMap<String, post>,
    }


    impl socialMediaPlatform {
        fn new() -> Self {
            Self {
                posts: HashMap::new(),
            }

        }

        


        fn post_content(&mut self, content: &str) -> String {
            let post = post::new(content);
            let id = post.id.clone();
            self.posts.insert(id.clone(), post);
            id
        }


       fn get_post(&self, id: &str) -> Option<&post>{
        self.posts.get(id)
       }

       fn add_comment(&mut self, id: &str, comment: &str) {
        if let Some(post) = self.posts.get_mut(id) {
            post.add_comment(comment);
        }

    }

        fn like_post(&mut self, id: &str) {
            if let Some(post) = self.posts.get_mut(id) {
                post.like();
            }
        }

        
        fn dislike_post(&mut self, id: &str) {
            if let Some(post) = self.posts.get_mut(id) {
                post.dislike();
            }
        }


        fn share_post(&self, id: &str) -> Option<String> {
            if self.posts.contains_key(id) {
                Some(format!("https://socialmedia.com/posts/{}", id))
            } else {
                None
            }
        }



       }


   

        fn main() {

        let mut platform = socialMediaPlatform::new();
    
       

        let post_id = platform.post_content("Hello, world!");
        println!("posted content with ID: {}", post_id);
    
       
        platform.like_post(&post_id);
        println!("posted content with ID: {:?}", platform.get_post(&post_id));

        platform.add_comment(&post_id, "Great post!");
        println!("Post after commenting: {:?}", platform.get_post(&post_id));
        // Share the post
  
        if let Some(link) = platform.share_post(&post_id) {
            println!("shareable link:{}", link);

        } else {
            println!("post not found!");
        }
    
       

        platform.dislike_post(&post_id);
        println!("Post after disliking {:?}", platform.get_post(&post_id));
    }
    




