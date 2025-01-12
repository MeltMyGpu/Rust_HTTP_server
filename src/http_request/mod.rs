use std::{
    error::Error, 
    io::{
        BufRead, 
        BufReader, 
        Write
    }, 
    net::{
        SocketAddr,  
        TcpStream
    },
};

use log::debug;
use request_errors::HttpRequestError;
pub mod request_errors;

#[derive(Debug)]
pub enum HttpRequestType {
    Get,
    //TODO: Implement more request types.
}

#[derive(Debug)]
pub struct HttpRequest {
    pub req_type: HttpRequestType,
    pub req_uri: String,
    req_peer_address: SocketAddr,
    req_stream: TcpStream,
}

/// Constructors 
impl HttpRequest {
    /* FIXME: 
        Code is generally messy because I started trying to handle errors.
        Gotta be a better way to do this.
        ![Clean up the code]
        ![Fix returned error to custom type]
     */
    /// Creates a new HttpRequest object from the provided TcpStream.
    pub fn new(req_stream: TcpStream) -> Result<Self, Box<dyn Error>> {

        let req_peer_address: SocketAddr = req_stream.peer_addr()?;
        let request: Vec<String> = HttpRequest::read_stream(&req_stream)?;
        
        debug!("Incoming request from: {} \n Request is: {:?}", req_peer_address, request);


        /* FIXME: 
            I shouldn't be indexing into this vector, this can cause index out of 
            bounds runtime errors if the vector is'nt the right size. 
            ![Refactor to use an iterator to avoid out of bounds errors]
         */
        let req_headers: Vec<_>= match request.first(){
            Some(x) => x.split(" ").collect(),
            None => todo!(), //TODO:
        };

        let req_type: HttpRequestType = match req_headers[0]{
            "GET" => HttpRequestType::Get,
            _ => todo!() //TODO:
        };


        let req_uri: String = if !req_headers[1].is_empty() {
            String::from(req_headers[1])
        } else {
            todo!() //TODO:
        }; 



        Ok(HttpRequest{
            req_type,
            req_uri,
            req_peer_address,
            req_stream,
        })
    }

}

/// Pub methods 
impl HttpRequest {
    /* FIXME:
        Currently returns a dynamic Error.
        ![Implement custom errors and have them returned, look at "thiserror" and "anyhow" crates]
     */
    /// Works as a wrapper to the TcpStream.write_all().
    /// Done so to ensure stream control by HttpRequest.
    pub fn write_all(&mut self, response: &str) -> Result<(), Box<dyn Error>>{
        match self.req_stream.write_all(response.as_bytes()){
            Ok(_) => Ok(()),
            Err(e) => {
                debug!("When writing to stream {}, error was encountered: {:?}", self.req_peer_address, e); //TODO: Change this?
                Err(e.into())
            },
        }
    }

}

/// Helpers 
impl HttpRequest {
    /* FIXME: 
        I think this actually only reads the headers from the request?
        Since the format of HTTP requests have the body following a blank line (I think).  
    */
    /// Reads data from stream and returns as a vec of Strings.
    fn read_stream(mut stream: &TcpStream) -> Result<Vec<String>, HttpRequestError>{
        let buffer = BufReader::new(&mut stream);
    
        let request: Vec<String> = buffer.lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
        
        match Vec::is_empty(&request){
            true => Err(HttpRequestError::RequestWrapError),
            false => Ok(request),
        }
    }
    
}