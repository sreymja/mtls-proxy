use serde_json::Value;
use crate::config::Config;
use crate::ui::handlers::RequestLogEntry;
use std::collections::HashMap;

pub fn dashboard_template(stats: &Value, _config: &Config) -> String {
    format!(
        r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Mock GPT-4o-mini Server Dashboard</title>
    <link rel="stylesheet" href="/ui/static/style.css">
    <link rel="icon" href="/ui/static/favicon.ico">
</head>
<body>
    <nav class="navbar">
        <div class="nav-brand">Mock GPT-4o-mini Server Dashboard</div>
        <div class="nav-links">
            <a href="/ui/dashboard" class="active">Dashboard</a>
            <a href="/ui/requests">Requests</a>
            <a href="/ui/health">Health</a>
        </div>
    </nav>
    
    <div class="container">
        <h1>Mock Server Dashboard</h1>
        
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
        
        <div class="charts-grid">
            <div class="chart-container">
                <h3>HTTP Methods</h3>
                <canvas id="methods-chart" width="400" height="300"></canvas>
            </div>
            
            <div class="chart-container">
                <h3>Status Codes</h3>
                <canvas id="status-chart" width="400" height="300"></canvas>
            </div>
        </div>
        
        <div class="config-section">
            <h3>Server Configuration</h3>
            <div class="config-grid">
                <div class="config-item">
                    <strong>Status:</strong> Running
                </div>
                <div class="config-item">
                    <strong>Version:</strong> 1.0.0
                </div>
            </div>
        </div>
        
        <div class="recent-activity">
            <h2>Recent Requests</h2>
            <div id="recent-requests" class="request-list">
                Loading...
            </div>
        </div>
    </div>
    
    <script src="/ui/static/script.js"></script>
    <script>
        // Initialize dashboard with stats
        const stats = {};
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

pub fn requests_template(logs: &[RequestLogEntry], _params: &HashMap<String, String>) -> String {
    let mut filters_html = String::new();
    
    // Build filter form
    filters_html.push_str(r#"<form method="GET" class="filters">"#);
    filters_html.push_str(r#"<input type="text" name="method" placeholder="HTTP Method" value="" />"#);
    filters_html.push_str(r#"<input type="number" name="status_code" placeholder="Status Code" value="" />"#);
    filters_html.push_str(r#"<input type="number" name="limit" placeholder="Limit" value="50" />"#);
    filters_html.push_str(r#"<button type="submit">Filter</button>"#);
    filters_html.push_str("</form>");
    
    let mut requests_html = String::new();
    
    for log in logs {
        let status_class = if log.response_status < 400 { "success" } else { "error" };
        
        requests_html.push_str(&format!(
            r#"
            <div class="request-entry {}">
                <div class="request-header">
                    <span class="method">{}</span>
                    <span class="path">{}</span>
                    <span class="status-code {}">{}</span>
                    <span class="duration">{}ms</span>
                    <span class="timestamp">{}</span>
                </div>
                <div class="request-details">
                    <div class="detail-row">
                        <strong>Request ID:</strong> <a href="/ui/request/{}">{}</a>
                    </div>
                    <div class="detail-row">
                        <strong>Client IP:</strong> {}
                    </div>
                    <div class="detail-row">
                        <strong>Body Size:</strong> {} bytes
                    </div>
                </div>
            </div>
            "#,
            status_class,
            log.method,
            log.path,
            status_class,
            log.response_status,
            log.response_time_ms,
            log.timestamp.format("%Y-%m-%d %H:%M:%S"),
            log.id,
            log.id,
            log.client_ip,
            log.body.as_ref().map(|b| b.len()).unwrap_or(0)
        ));
    }
    
    format!(
        r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Mock Server Requests</title>
    <link rel="stylesheet" href="/ui/static/style.css">
    <link rel="icon" href="/ui/static/favicon.ico">
</head>
<body>
    <nav class="navbar">
        <div class="nav-brand">Mock GPT-4o-mini Server Dashboard</div>
        <div class="nav-links">
            <a href="/ui/dashboard">Dashboard</a>
            <a href="/ui/requests" class="active">Requests</a>
            <a href="/ui/health">Health</a>
        </div>
    </nav>
    
    <div class="container">
        <h1>Request Logs</h1>
        
        {}
        
        <div class="requests-container">
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
        filters_html,
        requests_html
    )
}

pub fn request_detail_template(log_entry: &Option<RequestLogEntry>) -> String {
    match log_entry {
        Some(log) => {
            format!(
                r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Request Details - {}</title>
    <link rel="stylesheet" href="/ui/static/style.css">
    <link rel="icon" href="/ui/static/favicon.ico">
</head>
<body>
    <nav class="navbar">
        <div class="nav-brand">Mock GPT-4o-mini Server Dashboard</div>
        <div class="nav-links">
            <a href="/ui/dashboard">Dashboard</a>
            <a href="/ui/requests">Requests</a>
            <a href="/ui/health">Health</a>
        </div>
    </nav>
    
    <div class="container">
        <h1>Request Details</h1>
        
        <div class="request-detail-card">
            <div class="detail-section">
                <h3>Request Information</h3>
                <div class="detail-grid">
                    <div class="detail-item">
                        <strong>ID:</strong> {}
                    </div>
                    <div class="detail-item">
                        <strong>Method:</strong> {}
                    </div>
                    <div class="detail-item">
                        <strong>Path:</strong> {}
                    </div>
                    <div class="detail-item">
                        <strong>Timestamp:</strong> {}
                    </div>
                    <div class="detail-item">
                        <strong>Client IP:</strong> {}
                    </div>
                    <div class="detail-item">
                        <strong>Response Time:</strong> {}ms
                    </div>
                </div>
            </div>
            
            <div class="detail-section">
                <h3>Request Headers</h3>
                <pre class="json-display">{}</pre>
            </div>
            
            <div class="detail-section">
                <h3>Request Body</h3>
                <pre class="json-display">{}</pre>
            </div>
            
            <div class="detail-section">
                <h3>Response</h3>
                <div class="response-info">
                    <strong>Status:</strong> <span class="status-code {}">{}</span>
                </div>
                <pre class="json-display">{}</pre>
            </div>
        </div>
    </div>
    
    <script src="/ui/static/script.js"></script>
</body>
</html>
"#,
                log.id,
                log.id,
                log.method,
                log.path,
                log.timestamp.format("%Y-%m-%d %H:%M:%S"),
                log.client_ip,
                log.response_time_ms,
                serde_json::to_string_pretty(&log.headers).unwrap_or_else(|_| "{}".to_string()),
                log.body.as_ref().unwrap_or(&"No body".to_string()),
                if log.response_status < 400 { "success" } else { "error" },
                log.response_status,
                log.response_body.as_ref().unwrap_or(&"No response body".to_string())
            )
        }
        None => {
            format!(
                r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Request Not Found</title>
    <link rel="stylesheet" href="/ui/static/style.css">
    <link rel="icon" href="/ui/static/favicon.ico">
</head>
<body>
    <nav class="navbar">
        <div class="nav-brand">Mock GPT-4o-mini Server Dashboard</div>
        <div class="nav-links">
            <a href="/ui/dashboard">Dashboard</a>
            <a href="/ui/requests">Requests</a>
            <a href="/ui/health">Health</a>
        </div>
    </nav>
    
    <div class="container">
        <h1>Request Not Found</h1>
        <p>The requested log entry could not be found.</p>
        <a href="/ui/requests" class="btn">Back to Requests</a>
    </div>
</body>
</html>
"#
            )
        }
    }
}

pub fn health_template(health: &Value) -> String {
    let status = health["status"].as_str().unwrap_or("unknown");
    let status_class = if status == "healthy" { "healthy" } else { "unhealthy" };
    
    format!(
        r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Mock Server Health</title>
    <link rel="stylesheet" href="/ui/static/style.css">
    <link rel="icon" href="/ui/static/favicon.ico">
</head>
<body>
    <nav class="navbar">
        <div class="nav-brand">Mock GPT-4o-mini Server Dashboard</div>
        <div class="nav-links">
            <a href="/ui/dashboard">Dashboard</a>
            <a href="/ui/requests">Requests</a>
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
                    <strong>Available Models:</strong> {}
                </div>
                <div class="config-item">
                    <strong>Default Delay:</strong> {}ms
                </div>
                <div class="config-item">
                    <strong>Error Rate:</strong> {}%
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
        health["config"]["server_host"].as_str().unwrap_or("Unknown"),
        health["config"]["server_port"].as_u64().unwrap_or(0),
        health["config"]["max_connections"].as_u64().unwrap_or(0),
        health["config"]["available_models"].as_array()
            .map(|arr| arr.iter().filter_map(|v| v.as_str()).collect::<Vec<_>>().join(", "))
            .unwrap_or_else(|| "Unknown".to_string()),
        health["config"]["default_delay_ms"].as_u64().unwrap_or(0),
        health["config"]["error_rate_percent"].as_u64().unwrap_or(0)
    )
}
