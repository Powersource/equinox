use rsfml::graphics;
use collections::hashmap::HashMap;

pub type Key = ~str;

pub enum Resource {
	Font(graphics::Font),
	Color(graphics::Color),
}

pub struct ResourceLoader {
    priv resources: HashMap<Key, Resource>,

}

impl ResourceLoader {
	pub fn new() -> ResourceLoader {
		ResourceLoader { resources: HashMap::new() }
	}

	pub fn add_resource(&mut self, key: Key, resource: Resource) {
		self.resources.insert(key, resource);
	}

	pub fn get_font<'a>(&'a self, key: Key) -> &'a graphics::Font {
		let font = self.get_resource(&key);

		match *font {
			Font(ref f) => f,
			_ => fail!("unable to locate font {}", key),
		}
	}

	fn get_resource<'a>(&'a self, key: &Key) -> &'a Resource {
		match self.resources.find(key) {
			Some(resource) => resource,
			None => fail!("unable to find resource {}", *key),
		}
	}
}