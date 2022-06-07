use std::fs::File;



fn main(){
    let decoder = png::Decoder::new(File::open("res/character-tile-set.png").unwrap());
    let mut reader = decoder.read_info().unwrap();
    
    let mut buf = vec![0u8; reader.output_buffer_size()];
    reader.next_frame(buf.as_mut_slice()).unwrap();

    

    arch_specific();
}


#[cfg(not(target_arch = "mips"))]
fn arch_specific(){

}

#[cfg(target_arch = "mips")]
fn arch_specific(){

}