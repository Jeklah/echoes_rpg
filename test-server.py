#!/usr/bin/env python3
"""
Simple HTTP server for testing the Echoes RPG WASM build locally.
This server serves the game files and handles CORS properly for WASM.
"""

import http.server
import socketserver
import os
import sys
from pathlib import Path

class GameHTTPRequestHandler(http.server.SimpleHTTPRequestHandler):
    """Custom handler to serve WASM files with correct MIME types"""

    def end_headers(self):
        # Add CORS headers for local development
        self.send_header('Access-Control-Allow-Origin', '*')
        self.send_header('Access-Control-Allow-Methods', 'GET, POST, OPTIONS')
        self.send_header('Access-Control-Allow-Headers', '*')

        # Set correct MIME types for WASM and JS files
        if self.path.endswith('.wasm'):
            self.send_header('Content-Type', 'application/wasm')
        elif self.path.endswith('.js'):
            self.send_header('Content-Type', 'application/javascript')
        elif self.path.endswith('.html'):
            self.send_header('Content-Type', 'text/html; charset=utf-8')

        super().end_headers()

    def do_OPTIONS(self):
        """Handle CORS preflight requests"""
        self.send_response(200)
        self.end_headers()

    def log_message(self, format, *args):
        """Custom log formatting"""
        print(f"[{self.date_time_string()}] {format % args}")

def main():
    PORT = 8080

    # Change to the directory containing the built files
    script_dir = Path(__file__).parent

    # Check if we should serve from test-deploy or dist
    if (script_dir / "test-deploy").exists():
        web_dir = script_dir / "test-deploy"
        print(f"📁 Serving from test-deploy directory")
    elif (script_dir / "dist").exists():
        web_dir = script_dir / "dist"
        print(f"📁 Serving from dist directory")
    else:
        print("❌ Error: No built files found!")
        print("   Please run 'wasm-pack build --target web --out-dir pkg --no-typescript' first")
        print("   Then create a deployment directory with index.html and pkg/")
        sys.exit(1)

    # Check required files exist
    required_files = ["index.html", "pkg/echoes_rpg.js", "pkg/echoes_rpg_bg.wasm"]
    missing_files = []

    for file in required_files:
        if not (web_dir / file).exists():
            missing_files.append(file)

    if missing_files:
        print("❌ Error: Missing required files:")
        for file in missing_files:
            print(f"   - {file}")
        print("\n   Please ensure you have built the WASM package correctly.")
        sys.exit(1)

    # Change to web directory
    os.chdir(web_dir)

    # Find an available port
    for port in range(PORT, PORT + 10):
        try:
            with socketserver.TCPServer(("", port), GameHTTPRequestHandler) as httpd:
                print("🎮 Echoes RPG Test Server")
                print("=" * 50)
                print(f"🌐 Server running at: http://localhost:{port}")
                print(f"📂 Serving files from: {web_dir}")
                print("🔧 WASM and CORS headers configured")
                print("=" * 50)
                print("🎯 Open http://localhost:{} in your browser to play!".format(port))
                print("⏹️  Press Ctrl+C to stop the server")
                print()

                try:
                    httpd.serve_forever()
                except KeyboardInterrupt:
                    print("\n🛑 Server stopped by user")
                    break

        except OSError as e:
            if e.errno == 48 or e.errno == 98:  # Address already in use
                continue
            else:
                raise
    else:
        print(f"❌ Error: Could not find an available port starting from {PORT}")
        sys.exit(1)

if __name__ == "__main__":
    main()
