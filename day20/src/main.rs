fn main() {
    let input = include_str!("input");
    dbg!(run(input, 2));
    dbg!(run(input, 50));
}

struct Image {
    px: Vec<u8>,
    width: usize,
    bg: u8,
}

impl Image {
    fn height(&self) -> usize {
        self.px.len() / self.width
    }

    fn get(&self, x: i32, y: i32) -> usize {
        let val = if x < 0 || x as usize >= self.width {
            self.bg
        } else if y < 0 || y as usize >= self.height() {
            self.bg
        } else {
            self.px[self.width*(y as usize)+(x as usize)]
        };
        if val == b'.' { 0 } else { 1 }
    }
}

fn run(input: &str, count: usize) -> usize {
    let (key, image) = input.split_once("\n\n").unwrap();
    let key = key.as_bytes();
    let width = image.lines().next().unwrap().len();
    let mut image= Image {
        px: image.lines().map(str::as_bytes).flatten().cloned().collect(),
        width,
        bg: b'.',
    };

    for _step in 0..count {
        image = process(&image,key);
    }

    image.px.iter().filter(|b| **b == b'#').count()
}

fn process(image: &Image, key: &[u8]) -> Image {
    let width = image.width+2;
    let height = image.px.len()/image.width + 2;
    let mut px = vec![b'.';height*width];
    
    let bg = if image.bg == b'.' { b'#' } else { b'.' };

    for y in 0..height {
        for x in 0..width {
            let (xs,ys) = (x as i32 - 1, y as i32 - 1);
            let mut addr = 0usize;
            addr = 2*addr + image.get(xs-1,ys-1);
            addr = 2*addr + image.get(xs  ,ys-1);
            addr = 2*addr + image.get(xs+1,ys-1);
            addr = 2*addr + image.get(xs-1,ys  );
            addr = 2*addr + image.get(xs  ,ys  );
            addr = 2*addr + image.get(xs+1,ys  );
            addr = 2*addr + image.get(xs-1,ys+1);
            addr = 2*addr + image.get(xs  ,ys+1);
            addr = 2*addr + image.get(xs+1,ys+1);

            px[y*width+x] = key[addr];
        }
    }

    Image { px, width, bg }
}


