use serde_json::Value;

pub fn dashboard_template(stats: &Value) -> String {
    format!(
        r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>mTLS Proxy Dashboard</title>
    <link rel="stylesheet" href="/ui/static/style.css">
    <link rel="icon" href="/ui/static/favicon.ico">
</head>
<body>
    <nav class="navbar">
        <div class="nav-brand">mTLS Proxy Dashboard</div>
        <div class="nav-links">
            <a href="/ui/dashboard" class="active">Dashboard</a>
            <a href="/ui/logs">Logs</a>
            <a href="/ui/health">Health</a>
        </div>
    </nav>
    
    <div class="container">
        <h1>Dashboard</h1>
        
        <div class="stats-grid">
            <div class="stat-card">
                <h3>Total Requests (1h)</h3>
                <div class="stat-value" id="total-requests">{}</div>
            </div>
            
            <div class="stat-card">
                <h3>Success Rate</h3>
                <div class="stat-value" id="success-rate">{:.1}%</div>
            </div>
            
            <div class="stat-card">
                <h3>Avg Response Time</h3>
                <div class="stat-value" id="avg-response-time">{:.1}ms</div>
            </div>
            
            <div class="stat-card">
                <h3>Requests/Hour</h3>
                <div class="stat-value" id="requests-per-hour">{:.1}</div>
            </div>
        </div>
        
        <div class="chart-container">
            <canvas id="requests-chart" width="800" height="400"></canvas>
        </div>
        
        <div class="recent-activity">
            <h2>Recent Activity</h2>
            <div id="recent-logs" class="log-list">
                Loading...
            </div>
        </div>
    </div>
    
    <script src="/ui/static/script.js"></script>
    <script>
        // Initialize dashboard with stats
        const stats = {{}};
        updateDashboard(stats);
        
        // Auto-refresh every 30 seconds
        setInterval(() => {{
            fetch('/ui/api/stats')
                .then(response => response.json())
                .then(data => updateDashboard(data))
                .catch(console.error);
        }}, 30000);
    </script>
</body>
</html>
"#,
        stats["total_requests"].as_u64().unwrap_or(0),
        stats["success_rate"].as_f64().unwrap_or(0.0),
        stats["avg_response_time"].as_f64().unwrap_or(0.0),
        stats["requests_per_hour"].as_f64().unwrap_or(0.0)
    )
}

pub fn logs_template(
    logs: &[(
        crate::logging::RequestLog,
        Option<crate::logging::ResponseLog>,
    )],
    _params: &std::collections::HashMap<String, String>,
) -> String {
    let mut filters_html = String::new();

    // Build filter form
    filters_html.push_str(r#"<form method="GET" class="filters">"#);
    filters_html
        .push_str(r#"<input type="text" name="method" placeholder="HTTP Method" value="" />"#);
    filters_html.push_str(
        r#"<input type="number" name="status_code" placeholder="Status Code" value="" />"#,
    );
    filters_html
        .push_str(r#"<input type="number" name="limit" placeholder="Limit" value="100" />"#);
    filters_html.push_str(r#"<button type="submit">Filter</button>"#);
    filters_html.push_str("</form>");

    let mut logs_html = String::new();

    for (req, resp) in logs {
        let status_class = resp
            .as_ref()
            .map(|r| {
                if r.status_code < 400 {
                    "success"
                } else {
                    "error"
                }
            })
            .unwrap_or("unknown");

        let status_code = resp
            .as_ref()
            .map(|r| r.status_code.to_string())
            .unwrap_or_else(|| "N/A".to_string());

        let duration = resp
            .as_ref()
            .map(|r| format!("{}ms", r.duration_ms))
            .unwrap_or_else(|| "N/A".to_string());

        logs_html.push_str(&format!(
            r#"
            <div class="log-entry {}">
                <div class="log-header">
                    <span class="method">{}</span>
                    <span class="uri">{}</span>
                    <span class="status-code {}">{}</span>
                    <span class="duration">{}</span>
                    <span class="timestamp">{}</span>
                </div>
                <div class="log-details">
                    <div class="detail-row">
                        <strong>Client IP:</strong> {}
                    </div>
                    <div class="detail-row">
                        <strong>Request ID:</strong> {}
                    </div>
                    <div class="detail-row">
                        <strong>Body Size:</strong> {} bytes
                    </div>
                </div>
            </div>
            "#,
            status_class,
            req.method,
            req.uri,
            status_class,
            status_code,
            duration,
            req.timestamp.format("%Y-%m-%d %H:%M:%S"),
            req.client_ip,
            req.id,
            req.body_size
        ));
    }

    format!(
        r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>mTLS Proxy Logs</title>
    <link rel="stylesheet" href="/ui/static/style.css">
    <link rel="icon" href="/ui/static/favicon.ico">
</head>
<body>
    <nav class="navbar">
        <div class="nav-brand">mTLS Proxy Dashboard</div>
        <div class="nav-links">
            <a href="/ui/dashboard">Dashboard</a>
            <a href="/ui/logs" class="active">Logs</a>
            <a href="/ui/health">Health</a>
        </div>
    </nav>
    
    <div class="container">
        <h1>Request Logs</h1>
        
        {}
        
        <div class="logs-container">
            {}
        </div>
        
        <div class="pagination">
            <button onclick="loadMore()">Load More</button>
        </div>
    </div>
    
    <script src="/ui/static/script.js"></script>
</body>
</html>
"#,
        filters_html, logs_html
    )
}

pub fn health_template(health: &Value) -> String {
    let status = health["status"].as_str().unwrap_or("unknown");
    let status_class = if status == "healthy" {
        "healthy"
    } else {
        "unhealthy"
    };

    format!(
        r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>mTLS Proxy Health</title>
    <link rel="stylesheet" href="/ui/static/style.css">
    <link rel="icon" href="/ui/static/favicon.ico">
</head>
<body>
    <nav class="navbar">
        <div class="nav-brand">mTLS Proxy Dashboard</div>
        <div class="nav-links">
            <a href="/ui/dashboard">Dashboard</a>
            <a href="/ui/logs">Logs</a>
            <a href="/ui/health" class="active">Health</a>
        </div>
    </nav>
    
    <div class="container">
        <h1>Health Status</h1>
        
        <div class="health-status {}">
            <h2>Status: {}</h2>
        </div>
        
        <div class="health-details">
            <div class="detail-row">
                <strong>Last Request:</strong> {}
            </div>
            <div class="detail-row">
                <strong>Uptime:</strong> {}
            </div>
            <div class="detail-row">
                <strong>Version:</strong> {}
            </div>
        </div>
        
        <div class="config-section">
            <h3>Configuration</h3>
            <div class="config-grid">
                <div class="config-item">
                    <strong>Server Host:</strong> {}
                </div>
                <div class="config-item">
                    <strong>Server Port:</strong> {}
                </div>
                <div class="config-item">
                    <strong>Max Connections:</strong> {}
                </div>
                <div class="config-item">
                    <strong>Target URL:</strong> {}
                </div>
            </div>
        </div>
    </div>
    
    <script src="/ui/static/script.js"></script>
    <script>
        // Auto-refresh health status every 10 seconds
        setInterval(() => {{
            location.reload();
        }}, 10000);
    </script>
</body>
</html>
"#,
        status_class,
        status,
        health["last_request"].as_str().unwrap_or("Never"),
        health["uptime"].as_str().unwrap_or("Unknown"),
        health["version"].as_str().unwrap_or("Unknown"),
        health["config"]["server_host"]
            .as_str()
            .unwrap_or("Unknown"),
        health["config"]["server_port"].as_u64().unwrap_or(0),
        health["config"]["max_connections"].as_u64().unwrap_or(0),
        health["config"]["target_url"].as_str().unwrap_or("Unknown")
    )
}
