use std::io::Write;
use std::net::TcpStream;
use std::time::Duration;
use std::thread::sleep;
use std::fs::File;

fn main() -> std::io::Result<()> {
    //in_path and targets represent the file location and string which include
    //our list of target hosts
    let in_path = std::path::PathBuf::from("/etc/c2mon/c2mon-core.targets");
    let targets = std::fs::read_to_string(in_path)
        .expect("Could not read input file. Does /etc/c2mon/c2mon-core.targets exist?");
    //out_path is the path to which we will write our output file, out_string
    //will be what we write, and out_file is the file that we open at out_path
    let out_path = std::path::PathBuf::from("/etc/c2mon/c2mon-core.status");
    let mut out_string: String = "".to_string();
    //loop_time is the time in seconds we wait before running tests again.
    let loop_time = Duration::new(5, 0);
    
    //Now that we've finished setup, we want to run until the OS tells us to
    //quit.
    while 1 == 1 {
        for line in targets.lines(){
            //if the line contains a #, it's a comment line.
            if line.contains("#") == false{
                //host is more readable for the lines below, so we use that.
                let host = line;
                //test_tcp sounds like it should return true/false, but it
                //actually returns a string formated like:
                //"127.0.0.1:80         [✓]" 
                //or
                //"127.0.0.1:443        [X]"
                //This behavior may be changed at some point.
                let out_string_new_line = test_tcp(host);
                out_string = out_string + &out_string_new_line;
            }
        }
        output_status(out_string, &out_path)?;
        //We blank out_string so that it doesn't grow larger/repeat itself
        //in future iterations,
        out_string = "".to_string();
        //and then sleep for the time set earlier.
        sleep(loop_time);
    }
    Ok(())
}

//Output the value of 'text' into a file located at 'path'. Will truncate
//an existing file if something is located at 'path'.
fn output_status(text: String, path: &std::path::PathBuf) -> std::io::Result<()> {
    let mut out_file = File::create(path)?;
    write!(out_file, "{}", text)?;
    Ok(())
}

//If a connection can be mad at host, which is a string in the format IP:PORT,
//for example: 127.0.0.1:8080 . If connection is successful, output a string
//containing the IP:PORT and an indicator.
//
//Despite the name, I have kept this behavior because it makes main() more
//readable.
fn test_tcp(host: &str) -> String {
    let out_string: String;
    if let  Ok(_stream) = TcpStream::connect(host){
        //if the TCP connection is successful, add "host [✓] to the
        //next line in the out string.
        out_string = format!("{:20} [✓]\n", host);
    } else {
        //same as above, but for a failure case.
        out_string = format!("{:20} [X]\n", host);
    }
    out_string
}
