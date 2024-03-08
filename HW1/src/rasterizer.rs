use std::collections::HashMap;

use glam::{Mat4, Vec3, Vec4};




pub struct PosBufId{
    pub id: u32
}

pub struct IndBufId{
    pub id: u32
}

#[derive(Default)]
pub struct Rasterizer {
    width: i32,
    height: i32,

    model: Mat4,
    view: Mat4,
    projection: Mat4,

    pos_buf: HashMap<u32, Vec<Vec3>>,
    ind_buf: HashMap<u32, Vec<[usize; 3]>>,

    next_id: u32,
}

impl Rasterizer {
    pub fn new(w: i32, h: i32) -> Self{
        let mut res = Self {
            width: w,
            height: h,
            ..Default::default()
        };

        res
    }

    pub fn load_positions(&mut self, positions: Vec<Vec3>) -> PosBufId {
        let id = self.get_next_id();
        self.pos_buf.insert(id, positions);
        return PosBufId{ id };
    }

    // private fn
    fn  get_next_id(&mut self) -> u32 {
        self.next_id += 1;
        return self.next_id;
    }

    // the model matrix for rotating the triangle around the Z axis.
    //pub fn setModel(&mut self, &m) {
        //self.model = m;
    //}

    //pub fn setView(&mut self, &v) {
        //self.view = v;
    //}

    //pub fn setProjection(&mut self, &p) {
        //self.projection = p;
    //}

    //pub fn draw(&mut self, pos_buff: &PosBufId, ind_buff: &IndBufId){
        //let & buf = self.pos_buf[pos_buff.id];
        //let & ind = self.ind_buf[ind_buff.id];

        //let f1 = (100.0 - 0.1) / 2.0f;
        //let f2 = (100.0 + 0.1) / 2.0f;

        //mvp: Mat4 = self.projection * self.view * self.model;
        //for i in ind {
            //let t = Triangle::new();
            //let v = [
                //mvp * to_vec4(buf[i[0]], 1.0f),
                //mvp * to_vec4(buf[i[1]], 1.0f),
                //mvp * to_vec4(buf[i[2]], 1.0f)
            //];

            //for vi in v.iter_mut(){
                //*vi /= vi.w_axis;
            //}

            //// view
            //for vi in v.iter_mut(){
                //vi.x_axis = 0.5 * self.width as f32 * (vi.x_axis + 1.0);
                //vi.y_axis = 0.5 * self.height as f32 * (vi.y_axis + 1.0);
                //vi.z_axis = vi.z_axis * f1 + f2;
            //} 

            //for i in 0..3 {
                //t.setVertex(i, v[i].clone());
            //}

            //t.setColor(0, Rgb::new(255, 0, 0));
            //t.setColor(1, Rgb::new(0, 255, 0));
            //t.setColor(2, Rgb::new(0, 0, 255));
        //}

    //}
}

fn to_vec4(v3: &Vec3, w: f32) -> Vec4{
    Vec4::new(v3.x, v3.y, v3.z, w)
}