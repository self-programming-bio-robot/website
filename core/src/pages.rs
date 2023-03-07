pub struct Page {
    pub id: String,
    pub title: String,
    pub content: String,
    pub links: Vec<Link>,
}

pub struct Link {
    pub text: String,
    pub link: String,
}
