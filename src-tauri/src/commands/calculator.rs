#[tauri::command]
pub fn calculate_total(price: f64, quantity: u32) -> f64 {
    price * quantity as f64
}
