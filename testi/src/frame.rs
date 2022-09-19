

pub fn parse_simple_block(_buffer:&Vec<u8>){

    // let first_four = extract_part(&buffer,0,2);

    // // println!("as_bytes : {:?}",first_four);

    // let as_bits = bytes_array_to_bits_array(&first_four);

    // // println!("as_bits : {:?}",as_bits);

    // let mut cursor:usize = 0;

    // let frame_type = extract_part(&as_bits,0,0);    //0 for key frame 1 for interframe
    // let decoding_complexity = extract_part(&as_bits,1,3);   //0,1,2,3 normal simplenone none
    // let show_frame = extract_part(&as_bits,4,4);    //0 hidden 1 display
    // let partition_size = extract_part(&as_bits,5,23);

    // cursor = 3;

    // //key frame tag
    // if frame_type[0] == 0{
    //     let next_seven = extract_part(&buffer,3,9);
    //     cursor = 10;
    //     // println!("next_seven : {:?}",next_seven);
    //     let key_frame_bits = bytes_array_to_bits_array(&first_four);
    // }

    // println!("frame_type : {:?}",frame_type);
    // println!("decoding_complexity : {:?}",decoding_complexity);
    // println!("show_frame : {:?}",show_frame);
    // println!("partition_size : {:?}",partition_size);

    let decoder = init_bool_decoder(0);

    println!("{:?}",decoder);

}

#[derive(Debug,Clone)]
struct BoolDecoder{
    input:u8,
    range:u32,
    value:u32,
    bit_count:u32
}

#[allow(non_snake_case)]
fn read_bool(decoder:&mut BoolDecoder,prob:u8)->u8{

    let split:u32 = 1 + (((decoder.range-1) * prob as u32) >> 8);
    let SPLIT:u32 = split << 8;
    let retval:u8;

    if decoder.value >= SPLIT{
        retval = 1;
        decoder.range -= split;
        decoder.value -= SPLIT;
    } else {
        retval = 0;
        decoder.range = split;
    }

    while decoder.range < 128{
        decoder.value = decoder.value << 1;
        decoder.range = decoder.range << 1;
        if decoder.bit_count+1 == 8{
            decoder.bit_count = 0;
            decoder.value |= decoder.input as u32;
        }
    }

    return retval;

}

fn init_bool_decoder(start_position:u32)->BoolDecoder{
    let mut start_position = start_position;
    let mut value:u32 = 0;
    let mut i = 1;
    while i <= 2{
        value = (value << 8) | start_position;
        start_position+=1;
        i+=1;
    }
    BoolDecoder{
        value:value,
        input:start_position as u8,
        bit_count:0,
        range:255
    }
}

#[allow(dead_code)]
fn bytes_array_to_bits_array(pool:&Vec<u8>)->Vec<u8>{
    let mut collect = Vec::new();
    for i in pool{
        collect.append(&mut fill_bits(format!("{:b}",i)));
    }
    return collect;
}

#[allow(dead_code)]
fn extract_part(pool:&Vec<u8>,start:usize,end:usize)->Vec<u8>{
    let mut collect = Vec::new();
    for i in start..=end{
        collect.push(pool[i]);
    }
    return collect;
}

#[allow(dead_code)]
fn fill_bits(s:String)->Vec<u8>{
    let len = s.len();
    let left = 8-len;
    let mut make = String::new();
    for _ in 0..left{
        make.push_str(&"0");
    }
    make.push_str(&s);
    let mut collect = Vec::new();
    for c in make.chars(){
        collect.push(c.to_string().parse::<u8>().unwrap());
    }
    return collect;
}