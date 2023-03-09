pub mod bin;
pub mod data;

use std::rc::Rc;

use bin::websys_worker_handler::{FtlWorkerHandler, FtlWorkerHandlerInput, FtlWorkerHandlerOutput};
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_agent::{Bridge, Bridged};

pub struct App {
    input: String,
    input_reference: NodeRef,
    worker: Box<dyn Bridge<FtlWorkerHandler>>,
    output: String,
}

pub enum HandlerMessage {
    WorkerAction,
    ActiveWorker,
    WorkerMessage(FtlWorkerHandlerOutput),
}

impl Component for App {
    type Message = HandlerMessage;
    type Properties = ();

    fn create(context: &Context<Self>) -> Self {
        let callback = {
            let link = context.link().clone();
            move |e| link.send_message(HandlerMessage::WorkerMessage(e))
        };
        let worker = FtlWorkerHandler::bridge(Rc::new(callback));

        return Self {
            input: String::from(""), // TODO(Daniel): code ...
            input_reference: NodeRef::default(),
            worker: worker,
            output: String::from(""),
        };
    }

    fn update(self: &mut App, context: &yew::Context<Self>, message: <Self as yew::Component>::Message) -> bool {
        match message {
            HandlerMessage::WorkerAction => {}
            HandlerMessage::ActiveWorker => {
                if let Some(input) = self.input_reference.cast::<HtmlInputElement>() {
                    self.worker.send(FtlWorkerHandlerInput {
                        input: input.value() as String,
                    });
                }
            }
            HandlerMessage::WorkerMessage(output) => {
                self.output = format!("{:?}", output.output);
            }
        }

        return false;
    }
}
