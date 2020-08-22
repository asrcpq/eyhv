use crate::algebra::Point2f;
use crate::graphic_object::{GraphicObjects, GraphicObjectsIntoIter};

pub struct FpsIndicator {
	switch: bool,
	dt: f32,

	skip_count: u32,
	skip_frame: u32,

	graphic_objects: GraphicObjects,
}

impl FpsIndicator {
	pub fn new() -> FpsIndicator {
		FpsIndicator {
			switch: true,
			dt: 60.,
			skip_count: 0,
			skip_frame: 5,
			graphic_objects: Default::default(),
		}
	}

	pub fn switch(&mut self) {
		self.switch = !self.switch;
	}

	pub fn tick(&mut self, dt: f32) {
		if self.skip_count >= self.skip_frame {
			let dt = self.dt / self.skip_count as f32;
			let string = format!("{: >6.2}", 1. / dt);
			let len = string.len();
			self.graphic_objects = Default::default();
			for (i, ch) in string.chars().enumerate() {
				self.graphic_objects.extend(
					mray::fsd::fsd(ch)
						.zoom(20.)
						.shift(Point2f::from_floats(500. - (len - i) as f32 * 20., 10.)),
				)
			}

			self.skip_count = 1;
			self.dt = dt;
			return;
		}
		self.skip_count += 1;
		self.dt += dt;
	}

	pub fn graphic_objects_iter(&self) -> GraphicObjectsIntoIter {
		if self.switch {
			self.graphic_objects.clone()
		} else {
			Default::default()
		}
		.into_iter()
	}
}
