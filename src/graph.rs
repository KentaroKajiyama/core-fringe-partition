use std::collections::HashMap;

pub struct Graph {
    pub n: usize,
    pub adj: Vec<Vec<usize>>,
    pub degrees: Vec<usize>,
    pub nodes: Vec<String>,
    pub labels: Vec<String>, 
}

pub struct GraphBuilder {
    pub adj: Vec<Vec<usize>>,
    pub id_map: HashMap<String, usize>,
    pub nodes: Vec<String>,
    pub labels: Vec<String>, 
}

impl GraphBuilder {
    pub fn new() -> Self {
        GraphBuilder {
            adj: Vec::new(),
            id_map: HashMap::new(),
            nodes: Vec::new(),
            labels: Vec::new(),
        }
    }

    pub fn add_edge(&mut self, u_str: &str, v_str: &str, label: &str) {
        let u = self.get_id(u_str, label);
        let v = self.get_id(v_str, label);
        self.adj[u].push(v);
        self.adj[v].push(u);
    }

    fn get_id(&mut self, s: &str, label: &str) -> usize {
        if let Some(&id) = self.id_map.get(s) {
            // 既に存在する場合、もしラベルが "Botnet" なら上書きする（重要度が高いラベルを優先するロジック）
            if label.contains("Botnet") && !self.labels[id].contains("Botnet") {
                self.labels[id] = label.to_string();
            }
            id
        } else {
            let id = self.nodes.len();
            self.id_map.insert(s.to_string(), id);
            self.nodes.push(s.to_string());
            self.labels.push(label.to_string());
            self.adj.push(Vec::new());
            id
        }
    }

    pub fn build(self) -> Graph {
        let n = self.nodes.len();
        let mut degrees = vec![0; n];
        for i in 0..n {
            degrees[i] = self.adj[i].len();
        }
        Graph {
            n,
            adj: self.adj,
            degrees,
            nodes: self.nodes,
            labels: self.labels,
        }
    }
}

// k-coreを計算する
impl Graph {
    pub fn compute_k_core(&self) -> Vec<usize> {
        let n = self.nodes.len();
        let mut core = self.degrees.clone();
        let mut processed = vec![false; n];
        
        // 次数ごとにノードを整理する「バケット」を作る
        let max_deg = *core.iter().max().unwrap_or(&0);
        let mut buckets = vec![vec![]; max_deg + 1];
        for i in 0..n {
            buckets[core[i]].push(i);
        }

        let mut pointers = vec![0; max_deg + 1];

        // 低い次数から順に削る
        for k in 0..=max_deg {
            while let Some(&v) = buckets[k].get(pointers[k]) {
                pointers[k] += 1;
                if processed[v] { continue; }
                processed[v] = true;

                for &neighbor in &self.adj[v] {
                    if core[neighbor] > k {
                        core[neighbor] -= 1;
                        // 次数が下がったノードを下のバケットに「追加」する
                        // (古いバケットに残っていても processed チェックで弾かれる)
                        buckets[core[neighbor]].push(neighbor);
                    }
                }
            }
        }
        core
    }
}