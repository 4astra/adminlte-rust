use askama::Template;

#[derive(Template)]
#[template(path = "admin/index.html")] // path of admin dashboard
pub struct AdminDashboard<'a> {
    name: &'a str,
}

impl<'a> AdminDashboard<'a> {
    pub fn new(name: &'a str) -> Self {
        Self { name }
    }
}
