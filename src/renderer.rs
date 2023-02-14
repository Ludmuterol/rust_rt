pub fn render (width: u32, height: u32) -> Vec<u8>{
    let mut vec = vec![0u8; (width * height * 3).try_into().unwrap()];
    for x in vec.iter_mut() {
        *x = rand::random();
    }
    vec
}
