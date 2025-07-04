use godot::prelude::*;


#[derive(GodotClass)]
#[class(base = Object, init)]
pub struct Card {
    /// Unique card ID
    id: u32,
    /// Display title
    title: String,
    /// Optional parent card ID
    parent_id: Option<i64>,
    /// List of child card IDs
    children: Vec<u32>,
    /// Name of the bucket this card belongs to (within its parent)
    bucket: Option<String>,
    /// Ordered list of bucket names (used to organize child cards)
    buckets: Vec<String>,
}

#[godot_api]
impl Card {
    /// Internal helper to convert a Godot-style i64 to an Option<u32>
    // Godot doesn't take u32 and needs i64, cast 32 to ensure not negative then back to i64
    fn parent_id_from_godot(raw: i64) -> Option<i64> {
        if raw >= 0 {
            Some(raw as u32 as i64)
        } else {
            None
        }
    }

    #[func]
    pub fn new_card(title: String, parent_id: i64) -> Gd<Self> {
        let parent = Self::parent_id_from_godot(parent_id);

        Gd::from_init_fn(|_| Self {
            id: 0,
            title,
            parent_id: parent,
            children: Vec::new(),
            bucket: None,
            buckets: vec!["Default".to_string()],
        })
    }

    #[func]
    pub fn get_title(&self) -> String {
        self.title.clone()
    }

    #[func]
    pub fn set_title(&mut self, new_title: String) {
        self.title = new_title;
    }

    #[func]
    pub fn get_id(&self) -> u32 {
        self.id
    }

    #[func]
    pub fn get_parent_id(&self) -> i64 {
        self.parent_id.unwrap_or(-1)
    }

    #[func]
    pub fn set_parent_id(&mut self, new_parent_id: i64) {
        self.parent_id = Self::parent_id_from_godot(new_parent_id);
    }

    #[func]
    pub fn add_child(&mut self, new_child_id: i64) {
        if new_child_id < 0 {
        godot_warn!("Cannot add negative child ID: {}", new_child_id);
        return;
        }

        let id = new_child_id as u32;

        if self.children.contains(&id) {
            godot_warn!("Child {} is already in the list.", id);
            return;
        }

        self.children.push(id);
    }

    #[func]
    pub fn set_bucket(&mut self, bucket_name: String) {
        if bucket_name.is_empty() {
            self.bucket = None;
        } else {
            self.bucket = Some(bucket_name);
        }
    }

    #[func]
    pub fn add_bucket(&mut self, name: String) {
        if self.buckets.contains(&name) {
        godot_warn!("Bucket '{}' already exists.", name);
        return;
        }
        self.buckets.push(name);
    }

    #[func]
    pub fn remove_child(&mut self, child_id: i64) {
        if child_id < 0 {
            godot_warn!("Cannot remove negative child ID: {}", child_id);
            return;
        }

        let id = child_id as u32;

        if let Some(index) = self.children.iter().position(|&c| c == id) {
            self.children.remove(index);
        }  else {
        godot_warn!("Child ID {} not found in children list.", id);
        }
    }
}   
