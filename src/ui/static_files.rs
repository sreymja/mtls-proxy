pub const CSS: &str = r#"
/* Reset and base styles */
* {
    margin: 0;
    padding: 0;
    box-sizing: border-box;
}

body {
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, Cantarell, sans-serif;
    line-height: 1.6;
    color: #333;
    background-color: #f5f5f5;
}

/* Navigation */
.navbar {
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    color: white;
    padding: 1rem 2rem;
    display: flex;
    justify-content: space-between;
    align-items: center;
    box-shadow: 0 2px 4px rgba(0,0,0,0.1);
}

.nav-brand {
    font-size: 1.5rem;
    font-weight: bold;
}

.nav-links {
    display: flex;
    gap: 2rem;
}

.nav-links a {
    color: white;
    text-decoration: none;
    padding: 0.5rem 1rem;
    border-radius: 4px;
    transition: background-color 0.3s;
}

.nav-links a:hover,
.nav-links a.active {
    background-color: rgba(255,255,255,0.2);
}

/* Container */
.container {
    max-width: 1200px;
    margin: 0 auto;
    padding: 2rem;
}

/* Stats Grid */
.stats-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
    gap: 1.5rem;
    margin-bottom: 2rem;
}

.stat-card {
    background: white;
    padding: 1.5rem;
    border-radius: 8px;
    box-shadow: 0 2px 4px rgba(0,0,0,0.1);
    text-align: center;
}

.stat-card h3 {
    color: #666;
    font-size: 0.9rem;
    text-transform: uppercase;
    letter-spacing: 0.5px;
    margin-bottom: 0.5rem;
}

.stat-value {
    font-size: 2rem;
    font-weight: bold;
    color: #333;
}

/* Chart Container */
.chart-container {
    background: white;
    padding: 1.5rem;
    border-radius: 8px;
    box-shadow: 0 2px 4px rgba(0,0,0,0.1);
    margin-bottom: 2rem;
}

/* Logs */
.logs-container {
    background: white;
    border-radius: 8px;
    box-shadow: 0 2px 4px rgba(0,0,0,0.1);
    overflow: hidden;
}

.log-entry {
    padding: 1rem;
    border-bottom: 1px solid #eee;
    transition: background-color 0.3s;
}

.log-entry:hover {
    background-color: #f9f9f9;
}

.log-entry.success {
    border-left: 4px solid #28a745;
}

.log-entry.error {
    border-left: 4px solid #dc3545;
}

.log-entry.unknown {
    border-left: 4px solid #6c757d;
}

.log-header {
    display: flex;
    align-items: center;
    gap: 1rem;
    margin-bottom: 0.5rem;
}

.method {
    background: #007bff;
    color: white;
    padding: 0.25rem 0.5rem;
    border-radius: 4px;
    font-size: 0.8rem;
    font-weight: bold;
    min-width: 60px;
    text-align: center;
}

.uri {
    flex: 1;
    font-family: monospace;
    color: #666;
}

.status-code {
    padding: 0.25rem 0.5rem;
    border-radius: 4px;
    font-size: 0.8rem;
    font-weight: bold;
    min-width: 50px;
    text-align: center;
}

.status-code.success {
    background: #d4edda;
    color: #155724;
}

.status-code.error {
    background: #f8d7da;
    color: #721c24;
}

.duration {
    color: #666;
    font-size: 0.9rem;
}

.timestamp {
    color: #999;
    font-size: 0.8rem;
}

.log-details {
    font-size: 0.9rem;
    color: #666;
}

.detail-row {
    margin-bottom: 0.25rem;
}

/* Filters */
.filters {
    background: white;
    padding: 1rem;
    border-radius: 8px;
    box-shadow: 0 2px 4px rgba(0,0,0,0.1);
    margin-bottom: 1rem;
    display: flex;
    gap: 1rem;
    align-items: center;
}

.filters input {
    padding: 0.5rem;
    border: 1px solid #ddd;
    border-radius: 4px;
    font-size: 0.9rem;
}

.filters button {
    background: #007bff;
    color: white;
    border: none;
    padding: 0.5rem 1rem;
    border-radius: 4px;
    cursor: pointer;
    font-size: 0.9rem;
}

.filters button:hover {
    background: #0056b3;
}

/* Health Status */
.health-status {
    padding: 2rem;
    border-radius: 8px;
    margin-bottom: 2rem;
    text-align: center;
}

.health-status.healthy {
    background: #d4edda;
    color: #155724;
    border: 1px solid #c3e6cb;
}

.health-status.unhealthy {
    background: #f8d7da;
    color: #721c24;
    border: 1px solid #f5c6cb;
}

.health-details {
    background: white;
    padding: 1.5rem;
    border-radius: 8px;
    box-shadow: 0 2px 4px rgba(0,0,0,0.1);
    margin-bottom: 2rem;
}

.config-section {
    background: white;
    padding: 1.5rem;
    border-radius: 8px;
    box-shadow: 0 2px 4px rgba(0,0,0,0.1);
}

.config-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
    gap: 1rem;
    margin-top: 1rem;
}

.config-item {
    padding: 0.5rem;
    border-bottom: 1px solid #eee;
}

/* Pagination */
.pagination {
    text-align: center;
    margin-top: 2rem;
}

.pagination button {
    background: #007bff;
    color: white;
    border: none;
    padding: 0.75rem 1.5rem;
    border-radius: 4px;
    cursor: pointer;
    font-size: 1rem;
}

.pagination button:hover {
    background: #0056b3;
}

/* Recent Activity */
.recent-activity {
    background: white;
    padding: 1.5rem;
    border-radius: 8px;
    box-shadow: 0 2px 4px rgba(0,0,0,0.1);
}

.log-list {
    max-height: 400px;
    overflow-y: auto;
}

/* Responsive Design */
@media (max-width: 768px) {
    .navbar {
        flex-direction: column;
        gap: 1rem;
    }
    
    .nav-links {
        gap: 1rem;
    }
    
    .container {
        padding: 1rem;
    }
    
    .stats-grid {
        grid-template-columns: 1fr;
    }
    
    .log-header {
        flex-direction: column;
        align-items: flex-start;
        gap: 0.5rem;
    }
    
    .filters {
        flex-direction: column;
        align-items: stretch;
    }
    
    .config-grid {
        grid-template-columns: 1fr;
    }
}

/* Loading animation */
.loading {
    text-align: center;
    padding: 2rem;
    color: #666;
}

.loading::after {
    content: '';
    display: inline-block;
    width: 20px;
    height: 20px;
    border: 3px solid #f3f3f3;
    border-top: 3px solid #007bff;
    border-radius: 50%;
    animation: spin 1s linear infinite;
    margin-left: 0.5rem;
}

@keyframes spin {
    0% { transform: rotate(0deg); }
    100% { transform: rotate(360deg); }
}
"#;

pub const JS: &str = r#"
// Dashboard functionality
function updateDashboard(stats) {
    document.getElementById('total-requests').textContent = stats.total_requests || 0;
    document.getElementById('success-rate').textContent = (stats.success_rate || 0).toFixed(1) + '%';
    document.getElementById('avg-response-time').textContent = (stats.avg_response_time || 0).toFixed(1) + 'ms';
    document.getElementById('requests-per-hour').textContent = (stats.requests_per_hour || 0).toFixed(1);
    
    // Update chart if it exists
    updateChart(stats);
    
    // Load recent logs
    loadRecentLogs();
}

function updateChart(stats) {
    const canvas = document.getElementById('requests-chart');
    if (!canvas) return;
    
    const ctx = canvas.getContext('2d');
    
    // Simple bar chart
    ctx.clearRect(0, 0, canvas.width, canvas.height);
    
    const data = [
        { label: 'Total', value: stats.total_requests || 0, color: '#007bff' },
        { label: 'Success', value: stats.successful_requests || 0, color: '#28a745' },
        { label: 'Errors', value: stats.error_requests || 0, color: '#dc3545' }
    ];
    
    const maxValue = Math.max(...data.map(d => d.value));
    const barWidth = 60;
    const barSpacing = 40;
    const startX = 50;
    const startY = canvas.height - 50;
    
    data.forEach((item, index) => {
        const x = startX + index * (barWidth + barSpacing);
        const height = maxValue > 0 ? (item.value / maxValue) * (canvas.height - 100) : 0;
        const y = startY - height;
        
        // Draw bar
        ctx.fillStyle = item.color;
        ctx.fillRect(x, y, barWidth, height);
        
        // Draw label
        ctx.fillStyle = '#333';
        ctx.font = '12px Arial';
        ctx.textAlign = 'center';
        ctx.fillText(item.label, x + barWidth / 2, startY + 20);
        ctx.fillText(item.value, x + barWidth / 2, y - 10);
    });
}

function loadRecentLogs() {
    const container = document.getElementById('recent-logs');
    if (!container) return;
    
    fetch('/ui/api/logs?limit=10')
        .then(response => response.json())
        .then(logs => {
            container.innerHTML = logs.map(log => {
                const req = log[0];
                const resp = log[1];
                const statusClass = resp && resp.status_code < 400 ? 'success' : 'error';
                const statusCode = resp ? resp.status_code : 'N/A';
                
                return `
                    <div class="log-entry ${statusClass}">
                        <div class="log-header">
                            <span class="method">${req.method}</span>
                            <span class="uri">${req.uri}</span>
                            <span class="status-code ${statusClass}">${statusCode}</span>
                            <span class="timestamp">${new Date(req.timestamp).toLocaleString()}</span>
                        </div>
                    </div>
                `;
            }).join('') || '<div class="loading">No recent activity</div>';
        })
        .catch(error => {
            container.innerHTML = '<div class="loading">Error loading logs</div>';
            console.error('Error loading logs:', error);
        });
}

function loadMore() {
    const container = document.querySelector('.logs-container');
    if (!container) return;
    
    const currentLogs = container.querySelectorAll('.log-entry').length;
    
    fetch(`/ui/api/logs?limit=50&offset=${currentLogs}`)
        .then(response => response.json())
        .then(logs => {
            logs.forEach(log => {
                const req = log[0];
                const resp = log[1];
                const statusClass = resp && resp.status_code < 400 ? 'success' : 'error';
                const statusCode = resp ? resp.status_code : 'N/A';
                const duration = resp ? resp.duration_ms + 'ms' : 'N/A';
                
                const logElement = document.createElement('div');
                logElement.className = `log-entry ${statusClass}`;
                logElement.innerHTML = `
                    <div class="log-header">
                        <span class="method">${req.method}</span>
                        <span class="uri">${req.uri}</span>
                        <span class="status-code ${statusClass}">${statusCode}</span>
                        <span class="duration">${duration}</span>
                        <span class="timestamp">${new Date(req.timestamp).toLocaleString()}</span>
                    </div>
                    <div class="log-details">
                        <div class="detail-row">
                            <strong>Client IP:</strong> ${req.client_ip}
                        </div>
                        <div class="detail-row">
                            <strong>Request ID:</strong> ${req.id}
                        </div>
                        <div class="detail-row">
                            <strong>Body Size:</strong> ${req.body_size} bytes
                        </div>
                    </div>
                `;
                
                container.appendChild(logElement);
            });
        })
        .catch(error => {
            console.error('Error loading more logs:', error);
        });
}

// Auto-refresh functionality
function startAutoRefresh() {
    // Refresh dashboard every 30 seconds
    setInterval(() => {
        if (window.location.pathname === '/ui/dashboard') {
            fetch('/ui/api/stats')
                .then(response => response.json())
                .then(data => updateDashboard(data))
                .catch(console.error);
        }
    }, 30000);
}

// Initialize when page loads
document.addEventListener('DOMContentLoaded', function() {
    startAutoRefresh();
    
    // Add event listeners for interactive elements
    const filterForm = document.querySelector('.filters');
    if (filterForm) {
        filterForm.addEventListener('submit', function(e) {
            // Form submission is handled by the server
        });
    }
});
"#;

pub const FAVICON: &str = "\
<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"32\" height=\"32\" viewBox=\"0 0 32 32\">\
    <rect width=\"32\" height=\"32\" rx=\"4\" fill=\"#667eea\"/>\
    <path d=\"M8 8h16v4H8zM8 14h16v2H8zM8 18h16v2H8zM8 22h12v2H8z\" fill=\"white\"/>\
</svg>";
