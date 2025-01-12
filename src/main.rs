/*  
This file is here for mocking code and testing functionality, since I don't know exactly how this is 
going to end up working, I will mostly be getting stuff working in here and then refactoring it out as 
needs be. 
*/

use std::{
    fs, net::{
        TcpListener, 
        TcpStream
    }
};
use http_server::http_request::*;
use log::error;
use env_logger::Env;

fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();
    // env::set_var("RUST_BACKTRACE", "1");
    start_server();
}

pub fn start_server() -> () {
    let listener: TcpListener = TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in listener.incoming() {
        handle_connection(stream.unwrap());
    }
}

fn handle_connection(stream: TcpStream){
    
    let request = HttpRequest::new(stream);

    /* FIXME: 
        Currently failed to wrap requests don't get served an error page.
        These requests are just dropped, they aren't even logged because the original request 
        is owned by the HttpRequest object that failed to init.
        This may be fixable by having a custom error that wraps the original request stream, 
        so it is passed back to this scope to be handled properly.  
     */
    match request {
        Ok(request) => response_router(request),
        Err(e) => error!("Request fail to wrap and was dropped: {e}"),
    };

    
}

fn response_router(request: HttpRequest){
    if request.req_uri == "/" {
        response_page_index(request);
    }
    else {
        response_404(request);
    }
    
}

/* -------------------------------------------------------------------------- */
/*                               Basic responses                              */
/* -------------------------------------------------------------------------- */

/* TODO:
    Need to create abstraction that handles the creation and formatting of replies.
*/
fn response_page_index(mut request: HttpRequest) {
    let response_code = "HTTP/1.1 200 OK";
    let response_body = fs::read_to_string("html_responses/index.html").unwrap();
    let response_length = response_body.len();
    let response_msg = format!("{response_code}\r\n{response_length}\r\n\r\n{response_body}");
    request.write_all(&response_msg).unwrap();
}

fn response_404(mut request: HttpRequest){
    let response_code = "HTTP/1.1 404 Not found";
    let response_body = fs::read_to_string("html_responses/404.html").unwrap();
    let response_length = response_body.len();
    let response_msg = format!("{response_code}\r\n{response_length}\r\n\r\n{response_body}");
    request.write_all(&response_msg).unwrap();
}



/* -------------------------------------------------------------------------- */
/*                              will be extracted                             */
/* -------------------------------------------------------------------------- */
