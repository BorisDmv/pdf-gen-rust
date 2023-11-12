use printpdf::*;
use std::fs::File;
use std::io::BufWriter;

use mysql::prelude::*;
use mysql::Pool;
use std::env;

mod database {
    pub mod config;
}
use database::config::get_database_url;

fn main() {
    let pool = create_mysql_pool().expect("Failed to create MySQL connection pool");
    let mut conn = pool.get_conn().expect("Failed to get MySQL connection");

    /* This is if you want to pass argument for the query to be more dynamic */
    let args: Vec<String> = env::args().collect();

    // Check if an argument is provided
    if args.len() < 2 {
        eprintln!("Usage: {} <id>", args[0]);
        std::process::exit(1);
    }

    // Parse the ID from the command line argument
    let id: i32 = args[1].parse().expect("Invalid ID");

    let query_str = format!("SELECT id, name, role FROM userse WHERE id = {}", id);

    let result: Vec<(i32, String, String)> = conn
        .query_map(query_str, |(id, name, role)| (id, name, role))
        .unwrap();

    /* This is if you want to pass argument for the query to be more dynamic */

    /* This is if you want to have static query */

    //let result: Vec<(String, String, String)> = conn
    //        .query("SELECT id, name, role FROM users").unwrap();

    /* This is if you want to have static query */

    const WIDTH: f64 = 210.0;
    const HEIGHT: f64 = 297.0;
    const COL1_X: f64 = 10.0;
    //const COL2_X: f64 = 120.0;
    const ROW1_Y: f64 = 267.0;
    //const ROW2_Y: f64 = 1.0;
    //const ROW3_Y: f64 = 7.0;

    const MAX_ITEMS_PER_PAGE: usize = 20; // Set the maximum number of items per page
    let (doc, mut current_page, layer1) =
        PdfDocument::new("PDF_Document_title", Mm(WIDTH), Mm(HEIGHT), "Layer 1");
    let font = doc.add_builtin_font(BuiltinFont::TimesBold).unwrap();
    let mut y_position = ROW1_Y; // Initial y-position
    let mut item_count = 0;

    for (id, name, role) in &result {
        if item_count >= MAX_ITEMS_PER_PAGE {
            let (new_page, _) = doc.add_page(Mm(WIDTH), Mm(HEIGHT), "New Page");
            current_page = new_page;
            item_count = 0;
            y_position = ROW1_Y;
        }

        let current_layer = doc.get_page(current_page).get_layer(layer1);

        // Concatenate the text and call add_text for each item, incrementing y_position
        let text = format!("ID: {}, Name: {}, Role: {}", id, name, role);
        add_text(&current_layer, &font, COL1_X, y_position, &text, 16.0);

        // Increment the y_position and item_count
        y_position -= 10.0; // Adjust this value for the desired spacing
        item_count += 1;
    }

    doc.save(&mut BufWriter::new(File::create("file_name.pdf").unwrap()))
        .unwrap();
}

fn add_text(
    current_layer: &PdfLayerReference,
    font: &IndirectFontRef,
    x: f64,
    y: f64,
    text: &str,
    font_size: f64,
) {
    current_layer.begin_text_section();
    current_layer.set_font(font, font_size);
    current_layer.set_text_cursor(Mm(x), Mm(y));
    //current_layer.set_line_height(6.0);
    //current_layer.set_character_spacing(3.0);
    current_layer.write_text(text, font);
    current_layer.end_text_section();
}

fn create_mysql_pool() -> Result<Pool, mysql::Error> {
    let url = get_database_url();
    let opts = mysql::Opts::from_url(&url)?;
    Pool::new(opts)
}
