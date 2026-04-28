import { workspace, ExtensionContext } from 'vscode';

import {
	LanguageClient,
	LanguageClientOptions,
	ServerOptions,
	Executable
} from 'vscode-languageclient/node';

let client: LanguageClient;

export function activate(context: ExtensionContext) {
	// The server is implemented in Rust.
	// We'll point to the binary we've already built.
	// In a real extension, we would bundle this!
	const serverPath = workspace.getConfiguration('ryzelang').get<string>('languageServer.path', 'ryzelang-ls');
	
	const run: Executable = {
		command: serverPath,
		options: {
			env: {
				...process.env,
				// Add any necessary environment variables here
				RUST_LOG: 'debug'
			}
		}
	};

	// If the extension is launched in debug mode then the debug server options are used
	// Otherwise the run options are used
	const serverOptions: ServerOptions = {
		run,
		debug: run
	};

	// Options to control the language client
	const clientOptions: LanguageClientOptions = {
		// Register the server for Ryzelang documents
		documentSelector: [{ scheme: 'file', language: 'ryze' }],
		synchronize: {
			// Notify the server about file changes to '.ryze files contained in the workspace
			fileEvents: workspace.createFileSystemWatcher('**/*.ryze')
		}
	};

	// Create the language client and start the client.
	client = new LanguageClient(
		'ryze-ls',
		'Ryzelang Language Server',
		serverOptions,
		clientOptions
	);

	// Start the client. This will also launch the server
	client.start();
}

export function deactivate(): Thenable<void> | undefined {
	if (!client) {
		return undefined;
	}
	return client.stop();
}
