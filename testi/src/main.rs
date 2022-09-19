
mod io;
mod ebml;
mod frame;

use ebml::{Element,Document};

fn main() {

    let _f_h_h = format!("../chunks/heavy/file_1.webm");
    let _f_c_h_1 = format!("../chunks/heavy/file_2.webm");
    let _f_c_h_2 = format!("../chunks/heavy/file_3.webm");

    let _f_h_l = format!("../chunks/light/file_1.webm");
    let _f_c_l_1 = format!("../chunks/light/file_2.webm");
    let _f_c_l_2 = format!("../chunks/light/file_3.webm");

    let header_file = "_f_h_h";
    let header_file_type = _f_h_h.clone();

    let chunk_file = "_f_c_h_1";
    let chunk_file_type = _f_c_h_1.clone();

    

    // println!("{:?}",data);

    if false {

        let header_data = io::read_file(header_file_type.clone()).unwrap();
        let chunk_data = io::read_file(chunk_file_type.clone()).unwrap();

        let header_elements:Vec<Element>;
        match ebml::read_document(&header_data){
            Ok(mut v)=>{
                if true {
                    write_map(&header_file, &v);
                }
                if false {
                    write_document(&header_file, &v);
                }
                header_elements = v.get_header();
                if false{
                    write_webm(&header_file,&mut v);
                }
            },
            Err(_)=>{
                return;
            }
        }

        match ebml::read_document(&chunk_data){
            Ok(mut v)=>{
                v.add_header(&header_elements);
                if false{
                    write_webm(&chunk_file,&mut v);
                }
            },
            Err(_)=>{}
        }
    }

    if true{

        let data = io::read_file(_f_h_h.clone()).unwrap();
        match ebml::read_document(&data){
            Ok(v)=>{
                let elements = v.get_elements_by_id(&"35".to_string());

                if false {
                    for element in &elements{
                        println!("{:?}",element.value.binary()[0]);
                    }
                }

                if true {
                    let element = &elements[0];
                    let data = element.value.binary();
                    println!("{:?}",frame::parse_simple_block(&data));
                }

            },
            Err(_)=>{
                return;
            }
        }

    }


}

fn write_webm(file_type:&str,v:&mut Document){
    match io::write_file(format!("{}.webm",file_type),v.parse()){
        Ok(_)=>{
            println!(">>> file write successfull");
        },
        Err(e)=>{
            println!(">>> file write failed : {:?}",e);
        }
    }
}

fn write_map(file_type:&str,v:&Document){
    match io::write_file(format!("{}.txt",file_type),format!("{:#?}",v.get_map()).as_bytes().to_vec()){
        Ok(_)=>{
            println!(">>> file write successfull");
        },
        Err(e)=>{
            println!(">>> file write failed : {:?}",e);
        }
    }
}

fn write_document(file_type:&str,v:&Document){
    match io::write_file(format!("{}_document.txt",file_type),format!("{:#?}",v).as_bytes().to_vec()){
        Ok(_)=>{
            println!(">>> file write successfull");
        },
        Err(e)=>{
            println!(">>> file write failed : {:?}",e);
        }
    }
}

