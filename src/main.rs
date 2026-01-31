mod graph;
mod graphio;

use std::env;
use std::io::{BufRead};
use graph::GraphBuilder;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        return;
    }

    let file_path = &args[1];
    let reader = graphio::read_csv(file_path).expect("Failed to open file");

    let mut builder = GraphBuilder::new();
    let mut line_count = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        // ヘッダー行をスキップ (StartTimeで始まる行)
        if line_count == 0 && line.starts_with("StartTime") {
            line_count += 1;
            continue;
        }

        let parts: Vec<&str> = line.split(',').collect();
        
        // カラム数が足りているかチェック
        if parts.len() < 15 {
            continue; 
        }

        let src = parts[3];   // SrcAddr
        let dst = parts[6];   // DstAddr
        let label = parts[14]; // Label

        // グラフに追加
        builder.add_edge(src, dst, label);
        line_count += 1;
    }

    println!("Total lines processed: {}", line_count);
    
    // グラフ構築
    let graph = builder.build();
    println!("Graph built: {} nodes", graph.n);

    // K-Core分解の実行
    let cores = graph.compute_k_core();
    let max_k = cores.iter().max().unwrap_or(&0);
    println!("Max Core Number: {}", max_k);

    // --- 結果の分析と表示 ---
    println!("\n--- Top Core Nodes Analysis ---");
    
    // (CoreNumber, NodeID) のペアを作ってソート
    let mut nodes_with_k: Vec<(usize, usize)> = cores.iter().enumerate()
        .map(|(i, &k)| (k, i))
        .collect();
    
    // Core Number の降順でソート
    nodes_with_k.sort_by(|a, b| b.0.cmp(&a.0));

    // 上位20件を表示
    for (k, id) in nodes_with_k.iter().take(20) {
        println!("Core: {:>3} | IP: {:<15} | Label: {}", 
            k, 
            graph.nodes[*id], 
            graph.labels[*id]
        );
    }
}