use crate::data::{Comment, Post, PostThumbnail};

pub struct PostThumbnailMock;

impl PostThumbnailMock {
    pub fn get_hot_post_list() -> Vec<PostThumbnail> {
        use super::post;

        vec![
            post!("如何关闭 vim", "ont2unstho", "2024-01-20", false),
            post!("LLM 不能启动", "iolsksxvcn", "2024-01-19", false),
            post!("如何截断字符串", "2z86vsaw", "2024-01-18", false),
            post!("Rust 生命周期错误提示", "9lsqbsoige", "2024-01-17", true),
        ]
    }

    pub fn get_list() -> Vec<PostThumbnail> {
        use super::post;

        vec![
            post!("如何更换 Ubuntu apt 源", "ont2untho", "2024-01-20", false),
            post!(
                "Ubuntu apt update 很慢怎么办",
                "iolskxvcn",
                "2024-01-19",
                false
            ),
            post!(
                "把服务器 sudo rm -rf / 了怎么办",
                "2z86vaw",
                "2024-01-18",
                false
            ),
            post!("如何恢复误删的服务器数据", "9lsqbsoige", "2024-01-17", true),
            post!("DHCP 是什么", "3s54b65df1b3", "2024-01-16", true),
            post!("如何防止被 DDOS", "wa314z8v561a", "2024-01-15", true),
        ]
    }

    pub fn get_post(id: &str) -> Post {
        _ = id;
        Post {
            id: "如何更换 Ubuntu apt 源".to_string(),
            title: "如何更换 Ubuntu apt 源".to_string(),
            post_date_time: "2024-01-20".to_string(),
            solved: false,
            author: "Mike".to_string(),
            content: "Ubuntu 的源太慢了，想换一个更快的镜像，怎么换？有没有镜像推荐？Ubuntu 的源太慢了，想换一个更快的镜像，怎么换？有没有镜像推荐？".to_string()
        }
    }

    pub fn get_comments(post_id: String) -> Vec<Comment> {
        use super::comment;
        _ = post_id;

        vec![comment!(
            "12345",
            "Anduin",
            r#"
            编辑你的 /etc/apt/sources.list
            里面开头你会看到类似
            hk.archive.ubuntu.com
            把这个域名改成mirror.aiursoft.cn
            然后apt update
"#,
            "2024-01-03",
            true
        )]
    }
}

#[macro_export]
macro_rules! comment {
    ($id: expr, $author: expr, $content: expr, $date: expr, $accepted:expr) => {
        Comment {
            id: $id.to_string(),
            post_date_time: $date.to_string(),
            accepted: $accepted,
            author: $author.to_string(),
            content: $content.to_string(),
        }
    };
}

#[macro_export]
macro_rules! post {
    ($title: expr, $id: expr, $date: expr, $solved:expr) => {
        PostThumbnail {
            title: $title.to_string(),
            id: $id.to_string(),
            post_date_time: $date.to_string(),
            solved: $solved,
        }
    };
}
