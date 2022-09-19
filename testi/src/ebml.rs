const DEFAULT_SCHEMA_JSON: &str = include_str!("../schema.json");
use json::{parse,JsonValue};

#[derive(Debug,Clone)]
pub enum ElementValue{
    Null,
    U64(u64),
    I64(i64),
    F64(f64),
    Binary(Vec<u8>),
    BinaryRep(usize),
    String(String)
}

#[derive(Debug,Clone)]
pub struct Element{
    pub id:String,
    pub size:u64,
    pub data:Vec<u8>,
    pub children:Vec<Element>,
    pub value:ElementValue,
    pub name:String,
    pub data_type:String
}

impl Element{
    fn id_as_vint(&self)->Vec<u8>{
        let value_as_u64 = self.id.clone().parse::<usize>().unwrap();
        return make_vint(value_as_u64);
    }
    fn parse(&self)->Vec<u8>{
        return parse_element_to_bytes(&self);
    }
}

fn parse_element_to_bytes(element:&Element)->Vec<u8>{
    let mut collect = Vec::new();
    collect.append(&mut element.id_as_vint());
    if element.data_type == "m"{
        let mut children = Vec::new();
        for child in &element.children{
            children.append(&mut parse_element_to_bytes(&child));
        }
        let mut vint = make_master_vint(children.len());
        collect.append(&mut vint);
        collect.append(&mut children);
    } else {
        collect.append(&mut element.value.vint());
    }
    return collect;
}

#[derive(Debug,Clone)]
pub struct Document{
    pub elements:Vec<Element>
}

#[derive(Debug,Clone)]
pub enum DocumentMapElement{
    Pool(ElementWithChildren),
    Only(ElementWithOutChildren)
}

#[derive(Debug,Clone)]
pub struct ElementWithOutChildren{
    pub id:String,
    pub name:String,
    pub value:ElementValue
}

#[derive(Debug,Clone)]
pub struct ElementWithChildren{
    pub id:String,
    pub name:String,
    pub value:ElementValue,
    pub children:Vec<DocumentMapElement>
}

#[derive(Debug,Clone)]
pub struct DocumentMap{
    pub map:Vec<DocumentMapElement>
}

#[allow(dead_code)]
impl ElementValue{
    pub fn binary(&self)->Vec<u8>{
        match self{
            ElementValue::Binary(v)=>{return v.clone();},
            _=>{vec![]}
        }
    }
    pub fn u64(&self)->u64{
        match self{
            ElementValue::U64(v)=>{return v.clone();},
            _=>{0}
        }
    }
    pub fn i64(&self)->i64{
        match self{
            ElementValue::I64(v)=>{return v.clone();},
            _=>{0}
        }
    }
    pub fn f64(&self)->f64{
        match self{
            ElementValue::F64(v)=>{return v.clone();},
            _=>{0.0}
        }
    }
    pub fn string(&self)->String{
        match self{
            ElementValue::String(v)=>{return v.clone();},
            _=>{String::new()}
        }
    }
    pub fn vint(&self) -> Vec<u8>{
        match self{
            ElementValue::Null=>{
                vec![1,255,255,255,255,255,255]
            },
            ElementValue::Binary(v)=>{
                let mut collect:Vec<u8> = Vec::new();
                collect.append(&mut make_vint(v.len()));
                collect.append(&mut v.clone());
                collect
            },
            ElementValue::U64(v)=>{
                let mut collect:Vec<u8> = Vec::new();
                let mut as_bytes = v.to_be_bytes().to_vec();
                if v > &0{
                    loop{
                        if as_bytes.len() > 0{
                            if as_bytes[0] == 0{as_bytes.remove(0);} else {break;}
                        } else {break;}
                    }
                }
                collect.append(&mut make_vint(as_bytes.len()));
                collect.append(&mut as_bytes);
                collect
            },
            ElementValue::I64(v)=>{
                let mut collect:Vec<u8> = Vec::new();
                let mut as_bytes = v.to_be_bytes().to_vec();
                if v > &0{
                    loop{
                        if as_bytes.len() > 0{
                            if as_bytes[0] == 0{as_bytes.remove(0);} else {break;}
                        } else {break;}
                    }
                }
                collect.append(&mut make_vint(as_bytes.len()));
                collect.append(&mut as_bytes);
                collect
            },
            ElementValue::F64(v)=>{
                let mut collect:Vec<u8> = Vec::new();
                let mut as_bytes = v.to_be_bytes().to_vec();
                if v > &0.0{
                    loop{
                        if as_bytes.len() > 0{
                            if as_bytes[0] == 0{as_bytes.remove(0);} else {break;}
                        } else {break;}
                    }
                }
                collect.append(&mut make_vint(as_bytes.len()));
                collect.append(&mut as_bytes);
                collect
            },
            ElementValue::String(v)=>{
                let mut collect:Vec<u8> = Vec::new();
                let mut as_bytes = v.as_bytes().to_vec();
                collect.append(&mut make_vint(as_bytes.len()));
                collect.append(&mut as_bytes);
                collect
            },
            ElementValue::BinaryRep(_)=>{
                vec![]
            }
        }
    }
}

#[allow(dead_code)]
impl Document{
    pub fn parse(&mut self)->Vec<u8>{
        let mut parsed = Vec::new();
        for element in &self.elements{
            parsed.append(&mut element.parse());
        }
        return parsed;
    }
    pub fn add_header(&mut self,pool:&Vec<Element>){
        let mut collect = pool.to_vec();
        collect.append(&mut self.elements);
        self.elements = collect;
    }
    pub fn get_header(&self)->Vec<Element>{
        let mut collect = Vec::new();
        for element in &self.elements{
            if element.id != "35"{
                collect.push(element.clone());
            }
        }
        return collect;
    }
    pub fn get_elements_by_id(&self,id:&String)->Vec<Element>{
        let mut collect = Vec::new();
        for element in &self.elements{
            if &element.id == id{
                collect.push(element.clone());
            }
            let mut deep = get_elements_by_id_element(&element,&id);
            collect.append(&mut deep);
        }
        return collect;
    }
    pub fn get_map(&self)->DocumentMap{
        let mut collect = Vec::new();
        for element in &self.elements{
            let value:ElementValue;
            match &element.value{
                ElementValue::F64(_)=>{value = element.value.clone();},
                ElementValue::I64(_)=>{value = element.value.clone();},
                ElementValue::U64(_)=>{value = element.value.clone();},
                ElementValue::String(_)=>{value = element.value.clone();},
                ElementValue::Binary(v)=>{value = ElementValue::BinaryRep(v.len());},
                ElementValue::Null=>{value = element.value.clone();},
                _=>{value = ElementValue::Null;}
            }
            if element.children.len() > 0{
                collect.push(DocumentMapElement::Pool(ElementWithChildren{
                    id:element.id.clone(),
                    name:element.name.clone(),
                    value:value,
                    children:get_id_from_element_tree(&element)
                }));
            } else {
                collect.push(DocumentMapElement::Only(ElementWithOutChildren{
                    id:element.id.clone(),
                    name:element.name.clone(),
                    value:value,
                }));
            }
        }
        return DocumentMap{
            map:collect
        };
    }
}

fn get_id_from_element_tree(element:&Element)->Vec<DocumentMapElement>{
    let mut collect = Vec::new();
        for child in &element.children{
            let value:ElementValue;
            match &child.value{
                ElementValue::F64(_)=>{value = child.value.clone();},
                ElementValue::I64(_)=>{value = child.value.clone();},
                ElementValue::U64(_)=>{value = child.value.clone();},
                ElementValue::String(_)=>{value = child.value.clone();},
                ElementValue::Binary(v)=>{value = ElementValue::BinaryRep(v.len());},
                ElementValue::Null=>{value = child.value.clone();},
                _=>{value = ElementValue::Null;}
            }
            if element.children.len() > 0{
                collect.push(DocumentMapElement::Pool(ElementWithChildren{
                    id:child.id.clone(),
                    name:child.name.clone(),
                    value:value,
                    children:get_id_from_element_tree(&child)
                }));
            } else {
                collect.push(DocumentMapElement::Only(ElementWithOutChildren{
                    id:child.id.clone(),
                    name:child.name.clone(),
                    value:value,
                }));
            }
        }
        return collect;
}

#[allow(dead_code)]
fn get_elements_by_id_element(element:&Element,id:&String)->Vec<Element>{
    let mut collect = Vec::new();
        for children in &element.children{
            if &children.id == id{
                collect.push(children.clone());
            } 
            let mut deep = get_elements_by_id_element(&children,&id);
            collect.append(&mut deep);
        }
        return collect;
}

pub fn read_document(buffer:&Vec<u8>) -> Result<Document,&'static str>{
    match read_elements(&buffer, false){
        Ok(elements)=>{
            return Ok(Document{
                elements:elements
            });
        },
        Err(_)=>{
            return Err("failed-read_elements");
        }
    }
}

pub fn read_elements(buffer:&Vec<u8>,debug:bool) -> Result<Vec<Element>,&'static str>{

    if debug{
        // println!("reading");
    }

    let mut elements:Vec<Element> = Vec::new();

    let mut data = buffer.to_vec();

    if debug && false{
        println!("data before : {:?}",data.len());
    }

    while data.len() > 0{
        let (element,overflow) = read_element(&mut data,debug);
        // println!("{:?}",overflow.len());
        data = overflow;
        if debug && false{
            println!("element : {:?}",element);
        }
        if element.id == "0"{
            println!("no element found");
            break;
        }
        elements.push(element);
    }

    if debug && false{
        println!("data left : {:?}",data.len());
    }

    let schema:JsonValue;
    match parse(DEFAULT_SCHEMA_JSON){
        Ok(o)=>{schema = o;},
        Err(_)=>{
            return Err("failed-parse_schema");
        }
    }

    let mut collect:Vec<Element> = Vec::new();

    loop{
        if elements.len() > 0{

            let mut element = elements.remove(0);

            if debug && false{
                println!("{:?} {:?} {:?}",&element.id,schema[&element.id]["type"],element.data.len());
            }
            if schema[&element.id]["type"] == "m"{
                match read_elements(&element.data,true){
                    Ok(c)=>{
                        if debug && false{
                            println!("children : {:?}",c);
                        }
                        element.children = c;
                    },
                    Err(_)=>{
                        if debug && false{
                            println!("!!!!!!!!!!!! children failed");
                        }
                        return Err("parse-children-failed");
                    }
                }
            } else if schema[&element.id]["type"] == "u"{
                let parse = parse_u64(&element.data);
                element.value = ElementValue::U64(parse);
                // println!("{:?}",element.value);
            } else if schema[&element.id]["type"] == "s"{
                let parse = parse_i64(&element.data);
                element.value = ElementValue::I64(parse);
                // println!("{:?}",element.value);
            } else if schema[&element.id]["type"] == "f"{
                let parse = parse_f64(&element.data);
                element.value = ElementValue::F64(parse);
                // println!("{:?}",element.value);
            } else if schema[&element.id]["type"] == "b"{
                element.value = ElementValue::Binary(element.data.clone());
                // println!("{:?}",element.value);
            } else if schema[&element.id]["type"] == "8"{
                match String::from_utf8(element.data){
                    Ok(v)=>{
                        element.value = ElementValue::String(v);
                    },
                    Err(_)=>{}
                }
                // println!("{:?}",element.value);
            } else {
                println!("unsupported parsing {:?} {:?} {:?}",&element.id,schema[&element.id]["type"],element.data.len());
            }

            if schema.has_key(&element.id) && schema[&element.id].has_key("name"){
                element.name = schema[&element.id]["name"].to_string().clone();
            }
            if schema.has_key(&element.id) && schema[&element.id].has_key("type"){
                element.data_type = schema[&element.id]["type"].to_string().clone();
            }

            element.data = Vec::new();

            collect.push(element);

        } else {
            break;
        }
    }

    return Ok(collect);

    // return Err("no_error");

}

use std::io::Cursor;
use byteorder::{BigEndian, ReadBytesExt};

pub fn parse_u64(data:&Vec<u8>) -> u64{
    let mut local_data:Vec<u8> = Vec::new();
    for _ in 0..8-data.len(){
        local_data.push(0);
    }
    for i in data{
        local_data.push(*i);
    }
    let mut rdr = Cursor::new(&local_data);
    match rdr.read_u64::<BigEndian>(){
        Ok(v)=>{
            // if v ==0 {
            //     println!("local_data : {:?}",local_data);
            // }
            return v;
        },
        Err(e)=>{
            println!("parse u64 failed : {:?}",e);
            return 0;
        }
    }
}

pub fn parse_i64(data:&Vec<u8>) -> i64{
    let mut local_data:Vec<u8> = Vec::new();
    for _ in 0..8-data.len(){
        local_data.push(0);
    }
    for i in data{
        local_data.push(*i);
    }
    let mut rdr = Cursor::new(local_data);
    match rdr.read_i64::<BigEndian>(){
        Ok(v)=>{
            return v;
        },
        Err(e)=>{
            println!("parse i64 failed : {:?}",e);
            return 0;
        }
    }
}

pub fn parse_f64(data:&Vec<u8>) -> f64{
    let mut local_data:Vec<u8> = Vec::new();
    for _ in 0..8-data.len(){
        local_data.push(0);
    }
    for i in data{
        local_data.push(*i);
    }
    let mut rdr = Cursor::new(local_data);
    match rdr.read_f64::<BigEndian>(){
        Ok(v)=>{
            return v;
        },
        Err(e)=>{
            println!("parse f64 failed : {:?}",e);
            return 0.0;
        }
    }
}

pub fn read_element(buffer:&mut Vec<u8>,debug:bool)->(Element,Vec<u8>){
    let cursor = 0;
    if debug && false{
        println!("here 0 : cursor {:?} blen {:?}",cursor,buffer.len());
    }
    let (tag,cursor) = read_tag(&buffer, cursor);
    if debug && false{
        println!("here 1 tag {:?} cursor {:?}",tag,cursor);
    }
    let (size,cursor) = read_size(&buffer, cursor);
    if debug && false{
        println!("here 2 size {:?} cur {:?} fin cur {:?}",size,cursor,size as usize + cursor);
    }
    let (size_buffer,overflow) = buffer.split_at(size as usize + cursor);
    // if tag == 139690087{
    //     println!("--------------");
    //     println!("tag : {:?}",tag);
    //     println!("size : {:?}",size);
    //     println!("size_buffer : {:?}",size_buffer);
    //     println!("--------------");
    // }
    if debug && false{
        println!("here 3 {:?} {:?}",size_buffer.len(),overflow.len());
    }
    let (_,size_data) = size_buffer.split_at(cursor);
    
    return (Element{
        id:tag.to_string(),
        size:size,
        data:size_data.to_vec(),
        children:Vec::new(),
        value:ElementValue::Null,
        name:String::new(),
        data_type:String::new()
    },overflow.to_vec());
}

fn read_tag(buffer:&Vec<u8>,cursor:usize) -> (u64,usize){
    return read_v_int(&buffer,cursor,false);
}

fn read_size(buffer:&Vec<u8>,cursor:usize) -> (u64,usize){
    return read_v_int(&buffer,cursor,true);
}

fn read_v_int(buffer:&Vec<u8>,cursor:usize,read_empty:bool) -> (u64,usize){

    let start = fill_bits(format!("{:b}",buffer[cursor]));

    let (width,mut pool) = find_width(&start);
    let mut bytes = Vec::new();
    let mut p_found = false;
    for i in cursor+1..=cursor+width as usize{
        let mut as_bytes = fill_bits(format!("{:b}",buffer[i]));
        bytes.push(buffer[i]);
        pool.append(&mut as_bytes);
        if !p_found && buffer[i] != 255{
            p_found = true;
        }
    }

    if buffer[cursor] == 1 && read_empty && !p_found{
        return (0,cursor+width as usize+1);
    }

    let len:u64 = pool.len() as u64 -1;
    let mut count:u64 = 0;
    let mut num:u64 = 0;
    let base:u64 = 2;
    
    for i in pool{
        let calc = (i as u64) * base.pow(len as u32-count as u32);
        count += 1;
        num += calc;
    }

    let local_cursor = cursor + width as usize + 1;

    return (num,local_cursor);

}

fn find_width(pool:&Vec<u8>)->(u8,Vec<u8>){
    let mut count:u8 = 0;
    for i in pool{
        if i == &1{
            break;
        }
        count += 1;
    }
    let mut collect = Vec::new();
    for i in count as usize+1..pool.len(){
        collect.push(pool[i]);
    }
    return (count,collect);
}

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

fn make_number_from_bits(pool:Vec<u8>)->u64{
    let len:u64 = pool.len() as u64 -1;
    let mut count:u64 = 0;
    let mut num:u64 = 0;
    let base:u64 = 2;
    for i in pool{
        let calc = (i as u64) * base.pow(len as u32-count as u32);
        count += 1;
        num += calc;
    }
    return num;
}

fn make_master_vint(num:usize)->Vec<u8>{
    if num ==0 {
        // return vec![0,0,0,0,0,0,0,0];
        return vec![1,255,255,255,255,255,255,255];
    }
    let mut collect:Vec<u8> = Vec::new();
    let mut as_bytes = num.to_be_bytes().to_vec();
    loop{
        if as_bytes[0] ==0{as_bytes.remove(0);} else {break;}
    }
    if as_bytes.len() == 0{
        return vec![0,0,0,0,0,0,0,0];
    }
    collect.push(as_bytes.len() as u8);
    let padding = 8-1-as_bytes.len();
    for _ in 0..padding{
        collect.push(0);
    }
    collect.append(&mut as_bytes);
    return collect;
}

fn make_vint(n:usize)->Vec<u8>{
    if n == 0{
        return vec![1,255,255,255,255,255,255,255];
    }
    // if n == 1{
    //     return vec![1,0,0,0,0,0,0];
    // }
    let as_bytes = n.to_be_bytes();
    let mut refine = Vec::new();
    for item in as_bytes.iter(){
        if item != &0{
            refine.push(item);
        }
    }
    let mut collect:Vec<u8> = Vec::new();
    let filled = fill_bits(format!("{:b}",refine[0]));
    let mut found = false;
    for item in filled{
        if found{
            collect.push(item);
        }
        if item == 1 && !found{
            collect.push(item);
            found = true;
        }
    }
    for i in 1..refine.len(){
        let mut filled = fill_bits(format!("{:b}",refine[i]));
        collect.append(&mut filled);
    }
    let base_bytes:u64 = (collect.len() as f64 / 8.0).ceil() as u64;
    let required_bytes = ((collect.len() as u64 + base_bytes) as f64 / 8.0).ceil() as u64;
    let mut header = Vec::new();
    for _ in 0..required_bytes-1{
        header.push(0);
    }
    header.push(1);
    let padding = (required_bytes as usize * 8) - collect.len() - header.len();
    for _ in 0..padding{
        header.push(0);
    }
    header.append(&mut collect);
    let mut parts = Vec::new();
    let mut part = Vec::new();
    for i in header{
        if part.len() == 8{
            parts.push(part);
            part = Vec::new();
            part.push(i);
        } else
        if part.len() < 8{
            part.push(i);
        }
    }
    parts.push(part);
    let mut bytes = Vec::new();
    for part in parts{
        bytes.push(make_number_from_bits(part) as u8);
    }
    return bytes;
}

// #[cfg(test)]
// mod tests {
//     #[test]
//     use crate::ebml::ElementValue;
//     fn check_element_value_parsing() {
//         if false{
//             let make = ElementValue::Binary(vec![1,2,3,4]).vint();
//             println!("{:?}",make);
//         }
//         if false{
//             let make = ElementValue::Null.vint();
//             println!("{:?}",make);
//         }
//         if false{
//             let make_u64 = ElementValue::U64(23432324234674).vint();
//             let make_i64 = ElementValue::I64(78967987).vint();
//             let make_f64 = ElementValue::F64(211233123.6556).vint();
//             println!("u64 : {:?} {:?}",make_u64,read_v_int(&make_u64, 0, false));
//             println!("i64 : {:?} {:?}",make_i64,read_v_int(&make_i64, 0, false));
//             println!("f64 : {:?} {:?}",make_f64,read_v_int(&make_f64, 0, false));
//         }
//         if false{
//             let make = ElementValue::String("OPUS".to_string()).vint();
//             println!("{:?}",make);
//         }
//     }
// }