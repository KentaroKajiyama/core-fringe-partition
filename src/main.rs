mod graph;
mod graphio;

use std::env;
use std::io::{BufRead};
use std::fs::File;
use std::io::Write;
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
    let total_edges: usize = graph.adj.iter().map(|neighbors| neighbors.len()).sum::<usize>() / 2;
    println!("Graph built: {} nodes, {} edges", graph.n, total_edges);

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

    // --- 可視化用 CSV の出力 ---
    
    // 1. ノードリストの出力 (ID, IPアドレス, K-Core値, ラベル)
    let mut node_file = File::create("viz/nodes.csv").expect("Failed to create nodes.csv");
    writeln!(node_file, "id,label,k_core,category").unwrap();
    for i in 0..graph.n {
        writeln!(
            node_file, 
            "{},{},{},{}", 
            i, 
            graph.nodes[i], 
            cores[i], 
            graph.labels[i].replace(',', ";") // CSV 壊れ防止
        ).unwrap();
    }

    // 2. エッジリストの出力 (Source ID, Target ID)
    let mut edge_file = File::create("viz/edges.csv").expect("Failed to create edges.csv");
    writeln!(edge_file, "source,target").unwrap();
    for u in 0..graph.n {
        for &v in &graph.adj[u] {
            // 無向グラフなので、重複を避けるために u < v の場合のみ出力
            if u < v {
                writeln!(edge_file, "{},{}", u, v).unwrap();
            }
        }
    }

    println!("CSV files for visualization created in viz/ directory.");

    // 上位20件を表示
    for (k, id) in nodes_with_k.iter().take(20) {
        println!("Core: {:>3} | IP: {:<15} | Label: {}", 
            k, 
            graph.nodes[*id], 
            graph.labels[*id]
        );
    }
}