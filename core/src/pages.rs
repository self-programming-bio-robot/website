pub struct Page<'page> {
    pub id: &'page str,
    pub title: String,
    pub content: String,
    pub links: Vec<&'page str>,
}
