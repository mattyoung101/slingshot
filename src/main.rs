/*
 * Copyright (c) 2023 Matt Young.
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
use log::info;
use simple_logger::SimpleLogger;
use slingshot::completion::CompletionProvider;
use slingshot::completion::SvParserCompletion;
use slingshot::diagnostics::DiagnosticProvider;
use slingshot::diagnostics::VerilatorDiagnostics;

#[derive(Debug)]
struct Backend {

}

#[tokio::main]
async fn main() {
    // initialise logging framework
    // TODO we will probably need to use a file-based logging service since we'll be using an stdio
    // based LSP, so logging to stdio will interfere with that
    SimpleLogger::new().init().unwrap();
    
    //let document = "module x; logic [15:0] y;\nendmodule\n";
    
    let document = r#"
module mymodule(
    input logic [15:0] a,
    input logic [15:0] b,
    output logic[31:0] c
);
    logic [15:0] something;

    always_comb begin
        c = a + b + something;
    end
endmodule;

    "#;

    let result = VerilatorDiagnostics::diagnose(document).unwrap();
    for entry in result.iter() {
        info!("{:?}", entry);
    }

    let result2 = SvParserCompletion::extract_tokens(document).unwrap();
    for entry in result2.iter() {
        info!("entry: {:?}", entry);
    }
}

