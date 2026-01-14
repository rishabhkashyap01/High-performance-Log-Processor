# High-Performance Hybrid Log Processor (Java & Rust)

A cross-language systems utility designed to bridge the high-level management capabilities of **Java** with the low-level safety and performance of **Rust**. This project demonstrates the use of **JNI (Java Native Interface)** to achieve high-speed string parsing and file I/O.

## 🚀 Key Features
- **Native Performance:** Offloads heavy-duty log parsing to Rust for zero-cost abstractions and memory safety.
- **Interoperability:** Implements a seamless bridge between the JVM and Native code.
- **Efficient I/O:** Utilizes Rust's `BufReader` for optimized line-by-line scanning of large datasets.
- **Modern Java Integration:** Compatible with modern JDKs (including handling restricted native access).

## 🛠 Tech Stack
- **Java:** Application logic and management layer.
- **Rust:** High-performance processing engine.
- **JNI (Java Native Interface):** The communication bridge.
- **Cargo:** Rust build system and package manager.

## 📁 Directory Structure
```text
high-performance-log-processor/
├── java-app/src/
│   └── LogProcessor.java    # Java entry point & Native declarations
├── rust-core/
│   ├── src/
│   │   └── lib.rs           # Rust performance logic
│   └── Cargo.toml           # Build configuration & JNI dependencies
└── app.log                  # Target log file for processing
```

## ⚙️ Setup and Installation
Prerequisites

JDK 22+ (configured with JAVA_HOME)

Rust & Cargo (installed via rustup)

Xcode Command Line Tools (for macOS linkers)

1. Build the Rust Native Library

Bash
```
cd rust-core
cargo build --release
```
This generates liblog_processor.dylib (macOS) in target/release.

2. Compile the Java Client

Bash 
```
cd ../java-app/src
javac LogProcessor.java
```

3. Run the Application

Navigate back to the root directory and run:

Bash
```
java --enable-native-access=ALL-UNNAMED \
     -Djava.library.path=./rust-core/target/release \
     -cp ./java-app/src LogProcessor
```

## 📊 Performance Context

This project was developed to explore the efficiency gap between managed and unmanaged languages. By delegating string contains-checks to Rust, the system avoids the overhead of Java's Garbage Collector during massive file scans, making it ideal for processing gigabyte-scale logs.


Developed by:<ins>Rishabh Kashyap</ins>