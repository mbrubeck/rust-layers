// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use layers::Layer;
use color::Color;
use geom::size::Size2D;
use geom::matrix::Matrix4;

pub struct Scene {
    pub root: Layer,
    pub size: Size2D<f32>,
    pub transform: Matrix4<f32>,
    pub background_color: Color
}

pub fn Scene(root: Layer, size: Size2D<f32>, transform: Matrix4<f32>) -> Scene {
    Scene {
        root: root,
        size: size,
        transform: transform,
        background_color: Color {
                  r: 0.38f32,
                  g: 0.36f32,
                  b: 0.36f32,
                  a: 1.0f32
              }
    }
}

impl Scene {
    // FIXME: Workaround for cross-crate bug regarding mutability of class fields
    pub fn set_transform(&mut self, new_transform: Matrix4<f32>) {
        self.transform = new_transform;
    }
}

