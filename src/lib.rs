pub mod bin;
pub mod data;

use std::rc::Rc;

use bin::websys_worker_handler::{HandleWorkerFtl, HandleWorkerFtlInput, HandleWorkerFtlOutput};
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_agent::{Bridge, Bridged};

pub struct App {
    input: String,
    input_reference: NodeRef,
    worker: Box<dyn Bridge<HandleWorkerFtl>>,
    output: String,
}

pub enum HandlerMessage {
    WorkerAction,
    ActiveWorker,
    WorkerMessage(HandleWorkerFtlOutput),
}

impl Component for App {
    type Message = HandlerMessage;
    type Properties = ();

    fn create(context: &Context<Self>) -> Self {
        let callback = {
            let link = context.link().clone();
            move |e| link.send_message(HandlerMessage::WorkerMessage(e))
        };
        let worker = HandleWorkerFtl::bridge(Rc::new(callback));

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
                    self.worker.send(HandleWorkerFtlInput {
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

    fn view(&self, ctx: &Context<Self>) -> Html {
        return html! {
            <>
                <div>
                    <input ref={self.input_reference.clone()} type="string" max="254"/>
                    <p>
                        <i>
                            {"Node Input Reference: Outputs \n\t"}
                        </i>
                    </p>
                    <table>
                        <th>{"Preprocess"}</th>
                        <tr>
                            <td>{&self.output}</td>
                        </tr>
                    </table>
                </div>
                <button onclick={ctx.link().callback(|_| HandlerMessage::WorkerAction)}>{ "S U B M I T" }</button>
            </>
        };
    }
}
