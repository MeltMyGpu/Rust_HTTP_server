/*  
This file is here for mocking code and testing functionality, since I don't know exactly how this is 
going to end up working, I will mostly be getting stuff working in here and then refactoring it out as 
needs be. 
*/

use std::{
    fs, 
    io::{BufRead, BufReader, Write}, 
    net::{TcpListener, TcpStream}
};
use log::{debug, info};
use env_logger::Env;

fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();
    start_server();
}

pub fn start_server() -> () {
    let listener: TcpListener = TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in listener.incoming() {
        handle_connection(stream.unwrap());
    }
}

fn handle_connection(mut stream: TcpStream){
    info!("Incoming request from: {}",&stream.peer_addr().unwrap());
    let buffer = BufReader::new(&mut stream);


    // basic data parse TODO: Extract to HttpRequest? 
    let http_request:Vec<_> = buffer.lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    debug!("Incoming request is : {:?}", http_request);
    let request = HttpRequest::new(http_request.as_slice());
    successful_connection_response(stream, request);

    
}

fn successful_connection_response(stream: TcpStream, request: HttpRequest){
    if request.req_uri == "/" {
        response_page_index(stream);
    }
    else {
        response_404(stream);
    }
    
}

fn response_page_index(mut stream: TcpStream) {
    let response_code = "HTTP/1.1 200 OK";
    let response_body = fs::read_to_string("html_responses/index.html").unwrap();
    let response_length = response_body.len();
    let response_msg = format!("{response_code}\r\n{response_length}\r\n\r\n{response_body}");
    stream.write_all(response_msg.as_bytes()).unwrap();
}

fn response_404(mut stream: TcpStream){
    let response_code = "HTTP/1.1 404 Not found";
    let response_body = fs::read_to_string("html_responses/404.html").unwrap();
    let response_length = response_body.len();
    let response_msg = format!("{response_code}\r\n{response_length}\r\n\r\n{response_body}");
    stream.write_all(response_msg.as_bytes()).unwrap();
}



/* -------------------------------------------------------------------------- */
/*                              will be extracted                             */
/* -------------------------------------------------------------------------- */
#[derive(Debug)]
pub enum HttpRequestType {
    Get,

}

#[derive(Debug)]
pub struct HttpRequest {
    req_type: HttpRequestType,
    req_uri: String,
    // http_ver: f32,
    // req_header: Vec<String>, //NOTE: This may later be changed to a struct to make accessing individual headers easier.
}
impl HttpRequest {
    pub fn new(request: &[String]) -> Self{
        let mut req_iter = request.iter();
        let headers: Vec<_>= req_iter.next().unwrap().split(" ").collect();
        let req_type = match headers[0]{
            "GET" => HttpRequestType::Get,
            _ => todo!()
        };

        let req_uri = if !headers[1].is_empty() {String::from(headers[1])} else {todo!()};

        HttpRequest{
            req_type,
            req_uri,
            // http_ver: TODO: ?
            // req_header: TODO: ?
        }
    }
}