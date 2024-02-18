// Add dependencies in your Cargo.toml
// [dependencies]
// calamine = "0.23.0"

use calamine::{Reader, open_workbook_auto, Xlsx, DataType};

fn main() {
    // Open the Excel file
    let mut workbook: Xlsx<_> = open_workbook_auto("guest_data.xlsx").expect("Cannot open file");

    // Read the first sheet
    if let Some(Ok(sheet)) = workbook.worksheet_range_at(0) {
        // Define structures to store data
        let mut orders_by_mobile: std::collections::HashMap<String, Vec<String>> = std::collections::HashMap::new();
        
        // Iterate over rows
        for row in sheet.rows() {
            // Assuming the data starts from the second row (index 1)
            if let Some(Ok(DataType::String(mobile))) = row.get(0) {
                if let Some(Ok(DataType::String(food))) = row.get(4) {
                    // Check if the mobile number already exists in the HashMap
                    let orders = orders_by_mobile.entry(mobile.clone()).or_insert(Vec::new());
                    // Add the food item to the orders list
                    orders.push(food.clone());
                }
            }
        }

        // Now you have orders grouped by mobile numbers
        // You can analyze this data further to find repeat orders, guest loyalty, etc.
        // For example, you can count the frequency of each food item for each mobile number.
        for (mobile, orders) in &orders_by_mobile {
            println!("Mobile: {}", mobile);
            let mut food_count: std::collections::HashMap<&String, usize> = std::collections::HashMap::new();
            for order in orders {
                *food_count.entry(order).or_insert(0) += 1;
            }
            println!("Food preferences:");
            for (food, count) in food_count {
                println!("{}: {}", food, count);
            }
        }
    }
}
