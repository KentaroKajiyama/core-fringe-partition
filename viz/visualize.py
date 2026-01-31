import pandas as pd
import networkx as nx
from pyvis.network import Network

# Rust が書き出した CSV を読み込む
nodes_df = pd.read_csv("nodes.csv")
edges_df = pd.read_csv("edges.csv")

# K-Core が高いもの、または Botnet に絞る (数万件を軽くするため)
# ここで表示件数を調整できます
filtered_nodes = nodes_df[(nodes_df['k_core'] > 5) | (nodes_df['category'].str.contains("Botnet"))]
node_ids = set(filtered_nodes['id'])

# エッジをフィルタリング (両端のノードが残っているものだけ)
filtered_edges = edges_df[edges_df['source'].isin(node_ids) & edges_df['target'].isin(node_ids)]

# 可視化
net = Network(height="750px", width="100%", bgcolor="#222222", font_color="white")
# --- nodes の追加部分 ---
for _, row in filtered_nodes.iterrows():
    color = "red" if "Botnet" in row['category'] else "skyblue"
    # row['id'] を int() でキャスト
    net.add_node(
        int(row['id']), 
        label=str(row['label']), 
        title=f"K-core: {row['k_core']}", 
        color=color
    )

# --- edges の追加部分 ---
for _, row in filtered_edges.iterrows():
    # source と target を int() でキャスト
    net.add_edge(
        int(row['source']), 
        int(row['target'])
    )

net.write_html("graph.html")