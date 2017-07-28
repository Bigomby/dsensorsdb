pub struct Application {
    name: String,
    description: String,
}

impl Application {
    pub fn get_name(&self) -> &str {
        &self.name
    }
}
