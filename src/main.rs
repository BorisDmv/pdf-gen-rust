use printpdf::*;
use std::fs::File;
use std::io::BufWriter;

use mysql::prelude::*;
use mysql::{self, OptsBuilder};

fn main() {
    let server = "localhost";
    let database = "test";
    let username = "root";
    let password = "";
    let port = 3306; // 3306 otherwise 3307 for ssh tunnel

    // Build options for the database connection
    let opts = OptsBuilder::new()
            .ip_or_hostname(Some(server))
            .db_name(Some(database))
            .user(Some(username))
            .pass(Some(password))
            .tcp_port(port)
            .ssl_opts(None);

    // Connect to the database
    let pool = mysql::Pool::new(mysql::Opts::from(opts)).unwrap();
    let mut conn = pool.get_conn().unwrap();

    let result: Vec<(String, String, String)> = conn
            .query("SELECT id, client, quarter FROM taxable_wages")
            .unwrap();
    
    
    const WIDTH: f64 = 210.0;
    const HEIGHT: f64 = 297.0;
    const COL1_X: f64 = 10.0;
    const COL2_X: f64 = 120.0;
    const ROW1_Y: f64 = 267.0;
    const ROW2_Y: f64 = 1.0;
    const ROW3_Y: f64 = 7.0;
    
    
    const MAX_ITEMS_PER_PAGE: usize = 20; // Set the maximum number of items per page
    let (doc, mut current_page, layer1) =
        PdfDocument::new("PDF_Document_title", Mm(WIDTH), Mm(HEIGHT), "Layer 1");
    let font = doc.add_builtin_font(BuiltinFont::TimesBold).unwrap();
    let mut y_position = ROW1_Y; // Initial y-position
    let mut item_count = 0;

    for (id, client, quarter) in &result {
        if item_count >= MAX_ITEMS_PER_PAGE {
            let (new_page, _) = doc.add_page(Mm(WIDTH), Mm(HEIGHT), "New Page");
            current_page = new_page;
            item_count = 0;
            y_position = ROW1_Y;
        }

        let current_layer = doc.get_page(current_page).get_layer(layer1);

        // Concatenate the text and call add_text for each item, incrementing y_position
        let text = format!("ID: {}, Client: {}, Quarter: {}", id, client, quarter);
        add_text(&current_layer, &font, COL1_X, y_position, &text,16.0);

        // Increment the y_position and item_count
        y_position -= 10.0; // Adjust this value for the desired spacing
        item_count += 1;
    }
        
        
    doc.save(&mut BufWriter::new(File::create("file_name.pdf").unwrap()))
    .unwrap();
}


fn add_text(current_layer: &PdfLayerReference, font: &IndirectFontRef, x: f64, y: f64, text: &str, font_size: f64) {
    current_layer.begin_text_section();
    current_layer.set_font(font, font_size);
    current_layer.set_text_cursor(Mm(x), Mm(y));
    //current_layer.set_line_height(6.0);
    //current_layer.set_character_spacing(3.0);
    current_layer.write_text(text, font);
    current_layer.end_text_section();
}