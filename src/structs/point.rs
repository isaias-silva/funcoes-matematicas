pub struct Point {
    pub x: i32,
    pub y: i32,
}
impl Point {
    pub const fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }
    pub fn calc_function(&self, function: fn(i32) -> i32) -> i32 {
        return function(self.x);
    }
    pub fn show_coordenate(&self){
      
      println!("coordenadas: ({},{})",self.x,self.y)
    }
}
