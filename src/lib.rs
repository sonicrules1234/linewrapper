#[cfg(target_os = "android")]
use macroquad::prelude::*;

#[cfg(target_os = "android")]
#[derive(Debug)]
pub struct LineWrapper {
    lines: Vec<String>,
    pub cols: usize,
    pub rows: usize,
    charh: f32,
    charw: f32,
    textp: TextParams,
}

#[cfg(not(target_os = "android"))]
#[derive(Debug)]
pub struct LineWrapper {}

#[cfg(target_os = "android")]
#[macro_export]
macro_rules! lw_println {
    ($self:expr, $t:expr, $( $x:expr ),* ) => {
        $self.println(format!($t, 
        $(
            $x, 
        )*
        ));
        $self.show_lines();
    
    };
}

#[cfg(not(target_os = "android"))]
#[macro_export]
macro_rules! lw_println {
    ($self:expr, $t:expr, $( $x:expr ),* ) => {
        println!($t, 
        $(
            $x, 
        )*
        );
    
    };
}

#[cfg(target_os = "android")]
impl LineWrapper {
    pub fn new() -> Self {
        let ttffont = macroquad::text::Font::default();
        let lines: Vec<String> = Vec::new();
        let chardims = measure_text("a", Some(ttffont), 20, 1.0);
        let charh = chardims.height + 10.0;
        let charw = chardims.width;
        let textp = TextParams {
            font: ttffont,
            font_size: 20,
            font_scale: 1.0,
            font_scale_aspect: 1.0,
            color: WHITE,
        };
        let mut wcount = 0;
        let mut hcount = 0;
        let sw = screen_width();
        let sh = screen_height();
        while wcount as f32 * charw < sw {
            wcount += 1;
        }
        wcount -= 1;
        while hcount as f32 * charh < sh {
            hcount += 1;
        }
        hcount -= 1;
        Self {
            lines: lines,
            cols: wcount,
            rows: hcount,
            charh: charh,
            charw: charw,
            textp: textp,
        }
    }
    pub fn println(&mut self, data: impl Into<String>) -> () {
        let d = data.into();
        let output = textwrap::fill(d.as_str(), (self.cols - 2) as usize).replace("\r", "");
        let lines = output.split("\n").collect::<Vec<&str>>();
        for line in lines {
            self.lines.push(line.to_string());
            if self.lines.len() > (self.rows - 2).into() {
                self.lines.remove(0);
            }
        }
    }
    pub fn show_lines(&self) -> () {
        clear_background(BLACK);
        //print!("{}", &self.lines.join("\n"));
        let mut startx = self.charh;
        for line in &self.lines {
            draw_text_ex(&line, self.charw, startx, self.textp);
            startx = startx + self.charh
        }
    }
}

#[cfg(not(target_os = "android"))]
impl LineWrapper {
    pub fn new() -> Self {
        Self {
            
        }
    }
}