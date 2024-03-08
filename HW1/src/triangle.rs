use glam::{Vec2, Vec3, Vec4};

pub struct Rgb(u8, u8, u8);

pub struct Triangle{
    pub v: [Vec3; 3], // 3 vertexes
    pub color: [Rgb; 3], // color at each vertex;
    pub texture_coords: [Vec2; 3], // texture u, v
    pub normal: [Vec3; 3],  // normal vector for each vertex;
}

impl Triangle{
    
    // set i-th vertex coordinates
    pub fn setVertex(&mut self, ind: usize, vertex: Vec3){
        self.v[ind] = vertex;
    }

    // set i-th vertex normal vector
    pub fn setNormal(&mut self, ind: usize, normal: Vec3){
        self.normal[ind] = normal;
    }

    // set i-th vertex color
    pub fn setColor(&mut self, ind: usize, rgb: Rgb){
        self.color[ind] = rgb;
    }

    // set i-th vertex texture coordinates
    pub fn setTextureCoord(&mut self,
        ind: usize,
        tex_coord: Vec2)
    {
        self.texture_coords[ind] = tex_coord;
    }

    pub fn toVec4(&self) -> [Vec4; 3]{
        return [
            Vec4::new(self.v[0].x, self.v[0].y, self.v[0].z, 1.0),
            Vec4::new(self.v[1].x, self.v[1].y, self.v[1].z, 1.0),
            Vec4::new(self.v[2].x, self.v[2].y, self.v[2].z, 1.0),
        ];
    }
}