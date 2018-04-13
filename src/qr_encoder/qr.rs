use qr_encoder::cell::{
    Cell,
    Point,
    CellType,
    Color
};

use qr_encoder::config::{QRConfig};
use qr_encoder::util::{set_color, get_index_value};

pub struct QR {
    pub body: Vec<Cell>
}

impl QR {
    pub fn setup(&mut self, config: &QRConfig) {
        for alignment_point in config.finder_points.iter() {
            let point = Point(alignment_point.0, alignment_point.1);
            config.apply_finder_patterns(&mut self.body, point);
            config.apply_separators(&mut self.body, *alignment_point);
        }

        if config.version != 1 {
            let alignment_points = config.get_alignment_points(&self.body);
            config.apply_alignment_patterns(&mut self.body, &alignment_points);
        }


        config.apply_reserve_format_areas(&mut self.body);
        config.apply_dark_module(&mut self.body);
        config.apply_timer_patterns(&mut self.body);

        // version information area
        if config.version > 6 {
            config.apply_version_information_areas(&mut self.body);
        }

        println!("LENGTH IS {}, SIZE IS {}, VERSION: {}", self.body.len(), config.size, config.version);
        println!("--- QR ENCODER READY FOR ENCODING ---");
    }
}
