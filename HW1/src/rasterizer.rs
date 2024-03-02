use std::collections::HashMap;

use glum::{Mat4};

use crate::triangle::{Triangle};



pub struct PosBufId{
    pub id: u64
}

pub struct IndBufId{
    pub id: u64
}

struct Rasterizer {
    width: i32,
    height: i32,

    model: Mat4,
    view: Mat4,
    projection: Mat4,

    pos_buf: HashMap<u32, Vec<Vec3>>,
    ind_buf: HashMap<u32, Vec<[usize; 3]>>,
}

impl Raterizer {
    pub fn new(w: i32, h: i32) -> Self{
        let mut res = Self {
            width: w,
            height: h,
        };

        res
    }

    // the model matrix for rotating the triangle around the Z axis.
    pub fn setModel(&mut self, &m) {
        self.model = m;
    }

    pub fn setView(&mut self, &v) {
        self.view = v;
    }

    pub fn setProjection(&mut self, &p) {
        self.projection = p;
    }

    pub fn draw(&mut self, pos_buff: &PosBufId, ind_buff: &IndBufId){
        let & buf = self.pos_buf[pos_buff.id];
        let & ind = self.ind_buf[ind_buff.id];

        let f1 = (100.0 - 0.1) / 2.0f;
        let f2 = (100.0 + 0.1) / 2.0f;

        mvp: Mat4 = self.projection * self.view * self.model;
        for i in ind {
            let t = Triangle::new();
            let v = [
                mvp * to_vec4(buf[i[0]], 1.0f),
                mvp * to_vec4(buf[i[1]], 1.0f),
                mvp * to_vec4(buf[i[2]], 1.0f)
            ];

            for vi in v.iter_mut(){
                *vi /= vi.w_axis;
            }

            // view
            for vi in v.iter_mut(){
                vi.x_axis = 0.5 * self.width as f32 * (vi.x_axis + 1.0);
                vi.y_axis = 0.5 * self.height as f32 * (vi.y_axis + 1.0);
                vi.z_axis = vi.z_axis * f1 + f2;
            } 

            for i in 0..3 {
                t.setVertex(i, v[i].clone());
            }

            t.setColor(0, Rgb(255.0, 0.0, 0.0));
        }

    }
}

fn to_vec4(v3: &Vec3, w: f32) -> Vec4{
    Vec4::new(v3.x_axis, v3.y_axis, v3.z_axis, w)
}