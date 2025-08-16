pub const CSS: &str = r#"
body {
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
    margin: 0;
    padding: 20px;
    background-color: #f5f5f5;
}

.container {
    max-width: 1200px;
    margin: 0 auto;
    background: white;
    border-radius: 8px;
    box-shadow: 0 2px 10px rgba(0,0,0,0.1);
    overflow: hidden;
}

.header {
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    color: white;
    padding: 20px;
    text-align: center;
}

.header h1 {
    margin: 0;
    font-size: 2em;
    font-weight: 300;
}

.stats-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
    gap: 20px;
    padding: 20px;
}

.stat-card {
    background: white;
    border: 1px solid #e1e5e9;
    border-radius: 8px;
    padding: 20px;
    text-align: center;
    box-shadow: 0 2px 4px rgba(0,0,0,0.05);
}

.stat-card h3 {
    margin: 0 0 10px 0;
    color: #6c757d;
    font-size: 0.9em;
    font-weight: 500;
    text-transform: uppercase;
    letter-spacing: 0.5px;
}

.stat-value {
    font-size: 2em;
    font-weight: 700;
    color: #495057;
}

.charts-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(400px, 1fr));
    gap: 20px;
    padding: 20px;
}

.chart-container {
    background: white;
    border: 1px solid #e1e5e9;
    border-radius: 8px;
    padding: 20px;
    box-shadow: 0 2px 4px rgba(0,0,0,0.05);
}

.chart-container h3 {
    margin: 0 0 15px 0;
    color: #495057;
    font-size: 1.1em;
}

.config-section {
    padding: 20px;
    border-top: 1px solid #e1e5e9;
}

.config-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
    gap: 15px;
}

.config-item {
    padding: 10px;
    background: #f8f9fa;
    border-radius: 4px;
    border-left: 4px solid #667eea;
}

.recent-activity {
    padding: 20px;
    border-top: 1px solid #e1e5e9;
}

.request-list {
    max-height: 400px;
    overflow-y: auto;
}

.request-entry {
    padding: 15px;
    border: 1px solid #e1e5e9;
    border-radius: 4px;
    margin-bottom: 10px;
    background: white;
}

.request-entry.success {
    border-left: 4px solid #28a745;
}

.request-entry.error {
    border-left: 4px solid #dc3545;
}

.request-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 10px;
}

.method {
    background: #007bff;
    color: white;
    padding: 2px 8px;
    border-radius: 3px;
    font-size: 0.8em;
    font-weight: 600;
}

.path {
    font-family: monospace;
    color: #495057;
    flex: 1;
    margin: 0 15px;
}

.status-code {
    padding: 2px 8px;
    border-radius: 3px;
    font-size: 0.8em;
    font-weight: 600;
}

.status-code.success {
    background: #28a745;
    color: white;
}

.status-code.error {
    background: #dc3545;
    color: white;
}

.timestamp {
    color: #6c757d;
    font-size: 0.9em;
}

.request-details {
    font-size: 0.9em;
    color: #6c757d;
}

.detail-row {
    margin-bottom: 5px;
}

.detail-row a {
    color: #007bff;
    text-decoration: none;
}

.detail-row a:hover {
    text-decoration: underline;
}

.filters {
    padding: 20px;
    border-bottom: 1px solid #e1e5e9;
    display: flex;
    gap: 10px;
    align-items: center;
}

.filters input {
    padding: 8px 12px;
    border: 1px solid #ced4da;
    border-radius: 4px;
    font-size: 14px;
}

.filters button {
    padding: 8px 16px;
    background: #007bff;
    color: white;
    border: none;
    border-radius: 4px;
    cursor: pointer;
    font-size: 14px;
}

.filters button:hover {
    background: #0056b3;
}

.loading {
    text-align: center;
    color: #6c757d;
    padding: 20px;
}

@media (max-width: 768px) {
    .stats-grid {
        grid-template-columns: 1fr;
    }
    
    .charts-grid {
        grid-template-columns: 1fr;
    }
    
    .request-header {
        flex-direction: column;
        align-items: flex-start;
        gap: 5px;
    }
}
"#;

pub const JS: &str = r#"
// Dashboard functionality
function updateDashboard(stats) {
    // Update total requests
    const totalRequests = document.getElementById('total-requests');
    if (totalRequests) {
        totalRequests.textContent = stats.total_requests || 0;
    }
    
    // Update success rate
    const successRate = document.getElementById('success-rate');
    if (successRate) {
        successRate.textContent = (stats.success_rate || 0).toFixed(1) + '%';
    }
    
    // Update average response time
    const avgResponseTime = document.getElementById('avg-response-time');
    if (avgResponseTime) {
        avgResponseTime.textContent = (stats.avg_response_time || 0).toFixed(1) + 'ms';
    }
    
    // Update requests per hour
    const requestsPerHour = document.getElementById('requests-per-hour');
    if (requestsPerHour) {
        requestsPerHour.textContent = (stats.requests_per_hour || 0).toFixed(1);
    }
}

// Request list functionality
function loadRequests() {
    const container = document.querySelector('.requests-container');
    if (!container) return;
    
    fetch('/ui/api/requests?limit=20')
        .then(response => response.json())
        .then(requests => {
            container.innerHTML = requests.map(req => {
                const statusClass = req.response_status < 400 ? 'success' : 'error';
                
                return `
                    <div class="request-entry ${statusClass}">
                        <div class="request-header">
                            <span class="method">${req.method}</span>
                            <span class="path">${req.path}</span>
                            <span class="status-code ${statusClass}">${req.response_status}</span>
                            <span class="timestamp">${new Date(req.timestamp).toLocaleString()}</span>
                        </div>
                    </div>
                `;
            }).join('') || '<div class="loading">No recent requests</div>';
        })
        .catch(error => {
            container.innerHTML = '<div class="loading">Error loading requests</div>';
            console.error('Error loading requests:', error);
        });
}

function loadMore() {
    const container = document.querySelector('.requests-container');
    if (!container) return;
    
    const currentRequests = container.querySelectorAll('.request-entry').length;
    
    fetch(`/ui/api/requests?limit=50&offset=${currentRequests}`)
        .then(response => response.json())
        .then(requests => {
            requests.forEach(req => {
                const statusClass = req.response_status < 400 ? 'success' : 'error';
                
                const requestElement = document.createElement('div');
                requestElement.className = `request-entry ${statusClass}`;
                requestElement.innerHTML = `
                    <div class="request-header">
                        <span class="method">${req.method}</span>
                        <span class="path">${req.path}</span>
                        <span class="status-code ${statusClass}">${req.response_status}</span>
                        <span class="duration">${req.response_time_ms}ms</span>
                        <span class="timestamp">${new Date(req.timestamp).toLocaleString()}</span>
                    </div>
                    <div class="request-details">
                        <div class="detail-row">
                            <strong>Request ID:</strong> <a href="/ui/request/${req.id}">${req.id}</a>
                        </div>
                        <div class="detail-row">
                            <strong>Client IP:</strong> ${req.client_ip}
                        </div>
                        <div class="detail-row">
                            <strong>Body Size:</strong> ${req.body ? req.body.length : 0} bytes
                        </div>
                    </div>
                `;
                
                container.appendChild(requestElement);
            });
        })
        .catch(error => {
            console.error('Error loading more requests:', error);
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

pub const FAVICON: &[u8] = b"<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"32\" height=\"32\" viewBox=\"0 0 32 32\"><rect width=\"32\" height=\"32\" rx=\"4\" fill=\"#ff6b6b\"/><path d=\"M8 8h16v4H8zM8 14h16v2H8zM8 18h16v2H8zM8 22h12v2H8z\" fill=\"white\"/><circle cx=\"24\" cy=\"8\" r=\"4\" fill=\"#ffd93d\"/></svg>";
