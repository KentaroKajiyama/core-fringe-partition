## Core Fringe Partition

This repository implements the core-fringe partitioning algorithm for graph analysis.
We use the binetflow dataset from the Stratosphere IPS dataset (ref : Garcia, Sebastian. Malware Capture Facility Project. Retrieved from https://stratosphereips.org).

The entry point of the program is `main.rs`.

### Usage

```bash
cargo run (-- release) -- <file_path>
```

### Example

```bash
cargo run --release -- dataset/icmp_ddos.binetflow
```


### Details

#### Dataset
The dataset is one of the CTU-13 dataset, which is a dataset of botnet traffic that was captured in the CTU University, Czech Republic, in 2011. The goal of the dataset was to have a large capture of real botnet traffic mixed with normal traffic and background traffic. Here, we use ICMP DDoS attack scenario. Core nodes are probably the nodes that are used as a botnet controller or target of the attack. We can distinguish the type of the node by the label of the node.

In main.rs, we use the GraphBuilder to build a graph from the binetflow dataset.
binetflow dataset has the following columns:

- StartTime
- Dur
- Proto
- SrcAddr
- Sport
- Dir
- DstAddr
- Dport
- State
- sTos
- dTos
- TotPkts
- TotBytes
- SrcBytes
- Label

We use the SrcAddr and DstAddr as nodes, and the Label as the edge label.


#### k-core algorithm

Then we compute the k-core of the graph. The k-core is the largest subgraph in which all nodes have degree at least k. We use the default value of k = 3. k-core is computed using the `compute_k_core` method of the Graph struct. The algorithm follows that :

1. Initialize the core number of each node to 0.
2. For each node, if its degree is at least k, set its core number to k and add it to the k-core.
3. For each node, if its degree is less than k, remove it from the graph.
4. Repeat steps 2 and 3 until no more nodes are removed.
5. Return the k-core.

By calculating the k-core, we can separate core nodes from fringe nodes.

#### Time complexity $O(m+n)$
We use bucket sort to sort the nodes by their degree.
When initializing the buckets, we use the maximum degree of the graph, and we check every node's degree to initialize the buckets. Then we check every node's exactly once to calculate the core number of each node. Also, we look at each edge once or twice to update the core number of each node (if the graph is directed, we look at each edge once, if the graph is undirected, we look at each edge twice).
Hence, the time complexity is O(n + m), where n is the number of nodes and m is the number of edges.

#### Space complexity $O(m+n)$
- adjacency list $O(m+n)$
- degree array $O(n)$
- bucket array $O(m+n)$
Hence, the space complexity is $O(m+n)$, where n is the number of nodes and m is the number of edges.


#### Experimental results

Here is the top 20 result of the k-core calculation in the ICMP DDoS attack scenario.

Total lines processed: 107252
Graph built: 41931 nodes
Max Core Number: 14719

--- Top Core Nodes Analysis ---
Core: 14719 | IP: 147.32.80.9     | Label: flow=From-Botnet-V52-1-UDP-DNS
Core: 14719 | IP: 147.32.84.138   | Label: flow=To-Background-UDP-CVUT-DNS-Server
Core: 4387 | IP: 147.32.96.69    | Label: flow=From-Botnet-V52-2-ICMP
Core: 4387 | IP: 147.32.84.165   | Label: flow=From-Botnet-V52-1-UDP-DNS
Core: 4304 | IP: 147.32.84.59    | Label: flow=Background-Established-cmpgw-CVUT
Core: 4246 | IP: 147.32.84.191   | Label: flow=From-Botnet-V52-2-UDP-DNS
Core: 2563 | IP: 147.32.85.25    | Label: flow=To-Background-UDP-CVUT-DNS-Server
Core: 2109 | IP: 147.32.84.164   | Label: flow=From-Normal-V52-Grill
Core: 1479 | IP: 147.32.85.34    | Label: flow=Background
Core: 892 | IP: 147.32.85.7     | Label: flow=Background-google-analytics12
Core: 689 | IP: 147.32.86.20    | Label: flow=To-Background-UDP-CVUT-DNS-Server
Core: 568 | IP: 76.13.114.90    | Label: flow=Background-Established-cmpgw-CVUT
Core: 561 | IP: 147.32.84.189   | Label: flow=Background
Core: 478 | IP: 147.32.80.13    | Label: flow=From-Background-CVUT-Proxy
Core: 478 | IP: 147.32.84.94    | Label: flow=Background-UDP-Established
Core: 372 | IP: 147.32.84.170   | Label: flow=To-Background-Stribrek
Core: 366 | IP: 147.32.86.122   | Label: flow=To-Background-UDP-CVUT-DNS-Server
Core: 322 | IP: 188.138.84.239  | Label: flow=Background-TCP-Established
Core: 311 | IP: 147.32.86.135   | Label: flow=Background
Core: 280 | IP: 147.32.84.118   | Label: flow=Background-TCP-Attempt