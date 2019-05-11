extern crate rusoto_core;
extern crate rusoto_ec2;

use rusoto_core::{Region};
use rusoto_ec2::{
    Filter, Ec2, Ec2Client,
};
use rusoto_ec2::{DescribeTagsRequest};

use std::env;
use std::process::Command;
use curl::easy::Easy;

//Curl the meta-data endpoint to get our instance-id
fn my_id() -> String {
  
let mut response: String = String::new();
let mut handle = Easy::new();
handle.url("http://169.254.169.254/latest/meta-data/instance-id").unwrap();
{
    let mut transfer = handle.transfer();
    transfer.write_function(|data| {
        response = String::from_utf8(Vec::from(data)).unwrap();        
        Ok(data.len())
    }).unwrap();
    transfer.perform().unwrap();
}

return response.trim_end().to_string();
}


fn tags() {
    let ec2 = Ec2Client::new(Region::UsEast1);
  
    let filter_by_id = Filter {
                        name: Some("resource-id".to_owned()),
                        values: Some(vec![my_id().to_owned()])
                    };

    let req = DescribeTagsRequest {
        dry_run: None,
        filters: Some(vec![ filter_by_id ]),
        max_results: Some(50),
        next_token: None
    };
    match ec2.describe_tags(req).sync() {
        Ok(output)  => {
            match output.tags {
 		   Some(tags) => {
                       for t in tags {
                           let tag_name = t.key.unwrap();
 			   let tag_value = t.value.unwrap();
                           env::set_var(tag_name, tag_value);
                       }
                   },
    		   None => println!("There appear to be no tags"),
            }
        },
        Err(error) => {
            println!("Error: {:?}", error);
        }
    };


}

fn execute() {

  let args: Vec<String> = env::args().collect();
  let my_cmd = &args[1].to_string();
  let cmd_opts = &args[2..];

  let mut child = Command::new(my_cmd)
                        .args(cmd_opts)
                        .spawn()
                        .expect("failed to execute child");
  
  let ecode = child.wait()
                 .expect("failed to wait on child");

  assert!(ecode.success());
}

fn main() {
    tags();
    execute();
}

