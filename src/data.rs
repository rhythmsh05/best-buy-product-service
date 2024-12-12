use crate::model::Product;
use crate::configuration::Settings;

pub fn fetch_products(_settings: &Settings) -> Vec<Product> {
    vec![
        Product {
            id: 1,
            name: "Sony Noise Cancelling Headphones".to_string(),
            price: 299.99,
            description: "Immerse yourself in your favorite music with Sony's premium noise-cancelling headphones. Featuring cutting-edge sound technology and long-lasting comfort, these headphones are perfect for travel, work, or relaxation.".to_string(),
            image: "/sony.png".to_string()
        },
        Product {
            id: 2,
            name: "Samsung Galaxy Tab S9".to_string(),
            price: 899.99,
            description: "Experience the ultimate versatility with the Samsung Galaxy Tab S9. Its crystal-clear AMOLED display and powerful processor make it perfect for work, play, and everything in between.".to_string(),
            image: "/galaxy.png".to_string()
        },
        Product {
            id: 3,
            name: "Dyson V15 Detect Vacuum Cleaner".to_string(),
            price: 749.99,
            description: "Keep your home spotless with the Dyson V15 Detect. Featuring laser dust detection and powerful suction, this vacuum takes the hassle out of cleaning, leaving your space sparkling.".to_string(),
            image: "/vaccum_cleaner.png".to_string()
        },
        Product {
            id: 4,
            name: "Apple MacBook Pro 14".to_string(),
            price: 1999.99,
            description: "Unleash your creativity with the Apple MacBook Pro 14\". Packed with the M2 Pro chip and a stunning Liquid Retina XDR display, this laptop is designed for professionals and enthusiasts alike.".to_string(),
            image: "/macbook_14.png".to_string()
        }
    ]
}
