use crate::model::Product;
use crate::configuration::Settings;

pub fn fetch_products(_settings: &Settings) -> Vec<Product> {
    vec![
        Product {
            id: 1,
            name: "Apple Watch Ultra 2".to_string(),
            price: 1299.99,
            description: "Premium smart watch built for extreme sports, fitness tracking, and outdoor adventures.".to_string(),
            image: "/apple%20watch%20ultra%202.jpg".to_string()
        },
        Product {
            id: 2,
            name: "Apple Watch Series 10".to_string(),
            price: 999.99,
            description: "Next generation smart watch with advanced health monitoring and sleek design.".to_string(),
            image: "/applewatch%20serios%2010.jpg".to_string()
        },
        Product {
            id: 3,
            name: "JBL Vibe Buds".to_string(),
            price: 149.99,
            description: "True wireless earbuds with deep bass and long battery life for everyday use.".to_string(),
            image: "/jbl%20vibe%20buds.jpg".to_string()
        },
        Product {
            id: 4,
            name: "Apple AirPods 4".to_string(),
            price: 199.99,
            description: "Latest AirPods with improved sound quality and noise cancellation.".to_string(),
            image: "/airpods4.jpg".to_string()
        },
        Product {
            id: 5,
            name: "MacBook Pro 16".to_string(),
            price: 3299.99,
            description: "High performance laptop for developers, designers, and power users.".to_string(),
            image: "/macbook%20pro16.jpg".to_string()
        },
        Product {
            id: 6,
            name: "HP Victus 15 Gaming Laptop".to_string(),
            price: 1599.99,
            description: "Powerful gaming laptop with high refresh rate display and modern GPU.".to_string(),
            image: "/hp%20victus%2015.jpg".to_string()
        },
        Product {
            id: 7,
            name: "Dell G15 Gaming Laptop".to_string(),
            price: 1799.99,
            description: "Performance focused gaming laptop for serious gamers.".to_string(),
            image: "/dellG15.jpg".to_string()
        },
        Product {
            id: 8,
            name: "Lenovo Legion 5 Pro".to_string(),
            price: 1899.99,
            description: "High end gaming laptop with premium thermal design and display.".to_string(),
            image: "/lenovo%20legion%205%20pro.jpg".to_string()
        },
        Product {
            id: 9,
            name: "iPad Pro 13".to_string(),
            price: 1499.99,
            description: "Large display tablet for productivity, creativity, and entertainment.".to_string(),
            image: "/ipadpro13.jpg".to_string()
        },
        Product {
            id: 10,
            name: "iPhone 17 Pro Max".to_string(),
            price: 1899.99,
            description: "Flagship smartphone with advanced camera system and top performance.".to_string(),
            image: "/iphone%2017%20pro%20max.jpg".to_string()
        }
    ]
}
