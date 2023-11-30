pub fn calc_bullet_direction(
    start_x: f32, 
    start_y: f32, 
    end_x: f32, 
    end_y: f32,     
) -> (f32, f32) {
    
    let direction_x = end_x - start_x;
    let direction_y = end_y - start_y;
    
    let squared_mag = (direction_x * direction_x) + (direction_y * direction_y);
    
    if squared_mag == 0.0 {
        return (start_x, start_y);
    }
    
    let inverse_mag = 1.0 / squared_mag.sqrt();
    let x_normilized = direction_x * inverse_mag;
    let y_normilized = direction_y * inverse_mag;
    
    (x_normilized, y_normilized)
}