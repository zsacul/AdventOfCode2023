use std::ops::{Add,Sub,Mul};
#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Vec3f {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3f {
    pub fn new(x:f64,y:f64,z:f64)->Self
    {
        Self
        {
            x,y,z
        }
    }

    pub const ZERO: Vec3f = Self {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    pub const ONE: Vec3f = Self {
        x: 1.0,
        y: 1.0,
        z: 1.0,
    };        
    
    #[allow(unused)]
    pub fn len(&self)->f64
    {
        ((self.x*self.x + self.y*self.y + self.z*self.z) as f64).sqrt()
    }

    pub fn length(v:&Vec3f)->f64
    {
        ((v.x*v.x + v.y*v.y + v.z*v.z) as f64).sqrt()
    }

    #[allow(unused)]
    pub fn normalize(&self)->Self
    {
        let l = self.len();
        if l>0.0
        {
            return Self {
                x : (self.x as f64 / l),
                y : (self.y as f64 / l),
                z : (self.z as f64 / l), 
            }
        }
        *self
    }

    #[allow(unused)]
    pub fn dot(a: &Vec3f, b: &Vec3f)->f64 
    {
        a.x * b.x + a.y * b.y + a.z * b.z
    }

    #[allow(unused)]
    pub fn cross(a: &Vec3f, b: &Vec3f)->Self
    {
        Vec3f {
            x: a.y * b.z - a.z * b.y,
            y: a.z * b.x - a.x * b.z,
            z: a.x * b.y - a.y * b.x,
        }
    }

    #[allow(unused)]
    pub fn add(&self,other:&Vec3f)->Self
    {
        Self
        {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }

    #[allow(unused)]
    pub fn sub(&self,other:&Vec3f)->Self
    {
        Self
        {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }

    fn tangents(n:Vec3f)->(Vec3f,Vec3f)
    {	
        //Vec3 r,
        let mut up = Vec3f::new(0.0,1.0,0.0);
        let mut n = n.normalize();
          
        let mut r  = Self::cross(&up,&n);//up_m.cross(up, viewDirection_m ).unitize();
        r  = r.normalize();
    
        if (Self::length(&r)-1.0).abs()<0.0001
        {
          up = Self::cross(&n,&r);
          up = up.normalize();
        }
          else
        {
           up = Vec3f::new( 0.0, 0.0, if n.y < 0.0 {1.0} else {-1.0} );
           r  = Self::cross(&up,&n);
           r  = r.normalize();
        }
            
        //if ((n[1]==1 || n[1]==-1) && n[0]==0 && n[2]==0)
        //{
        //	r = Vec3(1,0,0);		
        //}
     //   else r = cross(n,up);
    
        //unitize(r);
        //up = cross(n,r);
        //unitize(up);
    
        //u = up;
        //v = r;
        (up,r)
    }


    fn project_to_plane(point:Vec3f,n:Vec3f)->Vec3f
    {
        let res = point -n*Self::dot(&n,&point);
    
        //Vec3f u,v;
        let (u,v) = Self::tangents(n);
        return Vec3f::new( Self::dot(&n,&res), Self::dot(&u,&res), Self::dot(&v,&res) );
        //return ToLocal(res,n,u,v);
    }
    
    //Returns the closest point on a line to a given point
    fn closest_point_on_line(a:Vec3f,b:Vec3f,v_point:Vec3f)->Vec3f
    {
      //First, we create a vector from our end point vA to our point vPoint
      let vector1 = v_point - a;
    
      //Now we create a normalized direction vector from end point vA to end point vB
      let vector2 = b - a;

      //Now we use the distance formula to find the distance of the line segment
      let d : f64 = Self::length(&vector2);
      let vector2 = Self::normalize(&vector2);

      //Using the dot product, we project the vVector1 onto the vector vVector2. This essentially
      //gives us the distance of our projected vector from vA
      let t = Self::dot(&vector2, &vector1);
    
      //If our projected distance from vA, "t",  is greater than or equal to 0, it must be closest to the end point
      //vA.  So we return this end point.
      if t<=0.0 { return a;}
    
      //If our projected distance from vA, "t", is greater than or equal to the magnitude or distance of the line
      //segment, it must be closest to the end point vB, so we return vB.
      if t >=d { return b; }
    
      //Here we create a vector that is of length T and in the direction of vVector2
      //To find the closest point on the line, we just add vVector3 to the original end point vA
      a + vector2*t
    }
}

impl Add for Vec3f
{
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Vec3f
{
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Mul<f64> for Vec3f
{
    type Output = Self;

    fn mul(self, n: f64) -> Self {
        Self {
            x: self.x * n,
            y: self.y * n,
            z: self.z * n,
        }
    }
}
