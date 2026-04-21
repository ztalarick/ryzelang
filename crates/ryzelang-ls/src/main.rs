use dashmap::DashMap;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};
use tree_sitter::{Parser, Point};

#[derive(Debug)]
struct Backend {
    client: Client,
    document_map: DashMap<String, String>,
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            server_info: Some(ServerInfo {
                name: "ryzelang-ls".to_string(),
                version: Some("0.1.0".to_string()),
            }),
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),
                hover_provider: Some(HoverProviderCapability::Simple(true)),
                completion_provider: Some(CompletionOptions {
                    resolve_provider: Some(false),
                    trigger_characters: Some(vec!["Q".to_string(), "W".to_string(), "E".to_string()]),
                    ..Default::default()
                }),
                definition_provider: Some(OneOf::Left(true)),
                ..Default::default()
            },
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "Ryzelang Language Server initialized!")
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        self.document_map
            .insert(params.text_document.uri.to_string(), params.text_document.text);
        self.on_change(params.text_document.uri).await;
    }

    async fn did_change(&self, mut params: DidChangeTextDocumentParams) {
        self.document_map.insert(
            params.text_document.uri.to_string(),
            std::mem::take(&mut params.content_changes[0].text),
        );
        self.on_change(params.text_document.uri).await;
    }

    async fn hover(&self, params: HoverParams) -> Result<Option<Hover>> {
        let uri = params.text_document_position_params.text_document.uri;
        let position = params.text_document_position_params.position;

        if let Some(source) = self.document_map.get(uri.as_str()) {
            let mut parser = Parser::new();
            parser.set_language(tree_sitter_ryze::language()).unwrap();
            let tree = parser.parse(source.as_str(), None).unwrap();
            let point = Point::new(position.line as usize, position.character as usize);
            
            if let Some(node) = tree.root_node().named_descendant_for_point_range(point, point) {
                let text = node.utf8_text(source.as_bytes()).unwrap_or("");
                if let Some(docs) = get_spell_docs(text) {
                    return Ok(Some(Hover {
                        contents: HoverContents::Scalar(MarkedString::String(docs.to_string())),
                        range: None,
                    }));
                }
            }
        }
        Ok(None)
    }

    async fn completion(&self, _: CompletionParams) -> Result<Option<CompletionResponse>> {
        let completions = vec![
            "Q", "W", "E", "EW", "QQ", "QW", "WE", "WQ", "EQ", "WEQ", "WWW", "EE", "EWQ", "EQE", "EEW", "R"
        ]
        .iter()
        .map(|&combo| CompletionItem {
            label: combo.to_string(),
            kind: Some(CompletionItemKind::FUNCTION),
            detail: Some(get_spell_docs(combo).unwrap_or("Ryze Combo").to_string()),
            ..Default::default()
        })
        .collect();

        Ok(Some(CompletionResponse::Array(completions)))
    }

    async fn goto_definition(&self, params: GotoDefinitionParams) -> Result<Option<GotoDefinitionResponse>> {
        let uri = params.text_document_position_params.text_document.uri;
        let position = params.text_document_position_params.position;

        if let Some(source) = self.document_map.get(uri.as_str()) {
            let mut parser = Parser::new();
            parser.set_language(tree_sitter_ryze::language()).unwrap();
            let tree = parser.parse(source.as_str(), None).unwrap();
            let point = Point::new(position.line as usize, position.character as usize);
            
            if let Some(node) = tree.root_node().named_descendant_for_point_range(point, point) {
                let text = node.utf8_text(source.as_bytes()).unwrap_or("");
                
                if let Some(def_range) = self.find_definition(tree.root_node(), text, source.as_bytes()) {
                    return Ok(Some(GotoDefinitionResponse::Scalar(Location {
                        uri,
                        range: def_range,
                    })));
                }
            }
        }
        Ok(None)
    }
}

impl Backend {
    fn find_definition(&self, node: tree_sitter::Node, name: &str, source: &[u8]) -> Option<Range> {
        if node.kind() == "combo_op" && node.child_by_field_name("store").is_some() {
            if let Some(name_node) = node.child_by_field_name("name") {
                if name_node.utf8_text(source).unwrap_or("") == name {
                    return Some(Range {
                        start: Position::new(node.start_position().row as u32, node.start_position().column as u32),
                        end: Position::new(node.end_position().row as u32, node.end_position().column as u32),
                    });
                }
            }
        }
        
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            if let Some(range) = self.find_definition(child, name, source) {
                return Some(range);
            }
        }
        None
    }

    async fn on_change(&self, uri: Url) {
        let mut diagnostics = Vec::new();
        if let Some(source) = self.document_map.get(uri.as_str()) {
            let mut parser = Parser::new();
            parser.set_language(tree_sitter_ryze::language()).unwrap();
            let tree = parser.parse(source.as_str(), None).unwrap();
            
            self.collect_errors(tree.root_node(), &mut diagnostics);
        }

        self.client.publish_diagnostics(uri, diagnostics, None).await;
    }

    fn collect_errors(&self, node: tree_sitter::Node, diagnostics: &mut Vec<Diagnostic>) {
        if node.is_error() || node.is_missing() {
            let range = Range {
                start: Position::new(node.start_position().row as u32, node.start_position().column as u32),
                end: Position::new(node.end_position().row as u32, node.end_position().column as u32),
            };
            diagnostics.push(Diagnostic {
                range,
                severity: Some(DiagnosticSeverity::ERROR),
                message: "Syntax Error: Malformed structure or invalid symbol.".to_string(),
                ..Default::default()
            });
        }
        
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            self.collect_errors(child, diagnostics);
        }
    }
}

fn get_spell_docs(combo: &str) -> Option<&'static str> {
    match combo {
        "Q" => Some("Overload: Push 1 to the Rune Stack."),
        "W" => Some("Rune Prison: Pop and discard the top of the Rune Stack."),
        "E" => Some("Spell Flux: Duplicate the top value of the Rune Stack."),
        "EW" => Some("Flux Swap: Swap the top two values of the Rune Stack."),
        "QQ" => Some("Add: Pop top two, push their sum to the Rune Stack."),
        "QW" => Some("Subtract: Pop top two, subtract top from second-top, push result."),
        "WE" => Some("Multiply: Pop top two, push their product."),
        "WQ" => Some("Divide: Pop top two, divide second-top by top, push result."),
        "EQ" => Some("Logic: If top is 0, push 1. Else push 0. Unwraps Flux-Marked Runes."),
        "WEQ" => Some("Control Flow: Conditional execution using Rune Stack (cond) and Scroll Stack (true/false)."),
        "WWW" => Some("Rune Prison: Execution pause (Sleep) for X milliseconds."),
        "EE" => Some("Flux Mark: Mark top Rune with Flux (Option/Result pattern)."),
        "EWQ" => Some("Output (Int): Pop and print top Rune as an integer."),
        "EQE" => Some("Output (Char): Pop and print top Rune as an ASCII character."),
        "EEW" => Some("Input (Char): Read one ASCII character and push to the Rune Stack."),
        "R" => Some("Realm Warp: Bind the top Scroll Block to the preceding name."),
        _ => None,
    }
}

#[tokio::main]
async fn main() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(|client| Backend {
        client,
        document_map: DashMap::new(),
    });
    Server::new(stdin, stdout, socket).serve(service).await;
}
