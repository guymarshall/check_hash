<h2>check_hash</h2>

Pass it a hash of a file followed by the filename, to check the file is unaltered

<h2>What does it do?</h2>
It checks the hash that a file SHOULD have against the hash the file HAS.

<h2>How do I run it?</h2>
You need Rust installed to compile and run this program. Please follow the instructions to install it from: <a href="https://www.rust-lang.org/tools/install">https://www.rust-lang.org/tools/install</a>
<br>
Once installed, you simply download this project, extract it into a directory, open a terminal, navigate to the directory, and run <code>cargo build --release</code>. This will create an executable in <code>check_hash/target/release/</code> called <code>check_hash</code>. Just move that executable to another directory with a file that you want to check the hash of. If you run the program without any CLI arguments, it will give you instructions on how to run the program correctly.