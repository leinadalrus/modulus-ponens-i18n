use serde::{Deserialize, Serialize};
use yew_agent::{HandlerId, Public, WorkerLink};

pub struct FtlWorkerHandler {
    link: WorkerLink<Self>,
}

#[derive(Serialize, Deserialize)]
pub struct FtlWorkerHandlerInput {
    pub input: String,
}

#[derive(Serialize, Deserialize)]
pub struct FtlWorkerHandlerOutput {
    pub output: String,
}

impl yew_agent::Worker for FtlWorkerHandler {
    type Reach = Public<Self>;
    type Message = ();
    type Input = FtlWorkerHandlerInput;
    type Output = FtlWorkerHandlerOutput;

    fn create(link: WorkerLink<Self>) -> Self {
        return Self { link };
    }

    fn update(&mut self, message: Self::Message) {}

    fn connected(&mut self, _id: HandlerId) {}

    fn handle_input(&mut self, msg: Self::Input, id: HandlerId) {
        // this runs in a web worker
        // and does not block the main
        // browser thread!

        fn process_ftl_file_through(input: String) -> String {
            // process the file from an internal string (impure) function ...
            // ... into a `&'static str` type for unicode/utf-8 encoding into ftl file.
            let output: String = input;
            return output;
        }

        let output = Self::Output {
            output: process_ftl_file_through(msg.input).to_owned(),
        };

        self.link.respond(id, output);
    }

    fn disconnected(&mut self, _id: HandlerId) {}

    fn destroy(&mut self) {}

    fn name_of_resource() -> &'static str {
        "index.js"
    }

    fn resource_path_is_relative() -> bool {
        false
    }

    fn is_module() -> bool {
        false
    }
}