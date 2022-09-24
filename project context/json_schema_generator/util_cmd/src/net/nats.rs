// // Supposed to be used to send nats streaming commands


// use nats::{Connection, Message};
// use std::time::Duration;

// fn sample_message() {
//     let nc: Connection = nats::connect("0.0.0.0").unwrap();

//     // With a timeout.
//     let resp = nc.request_timeout("foo", "Help me?", Duration::from_secs(2)).unwrap();

//     // With multiple responses.
//     for msg in nc.request_multi("foo", "Help").unwrap().iter() {}

//     // Publish a request manually.
//     let reply = nc.new_inbox();
//     let rsub = nc.subscribe(&reply).unwrap();
//     nc.publish_request("foo", &reply, "Help me!").unwrap();
//     let response = rsub.iter().take(1);
// }



#[cfg(test)]
mod tests {
    #[test]
    fn test_sample_msg() {
        assert_eq!(2 + 2, 4);
    }
}
