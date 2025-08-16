use dioxus::prelude::*;
use dioxus_router::prelude::*;

use crate::app::Route;

#[component]
pub fn Layout() -> Element {
    let navigator = use_navigator();
    let _route = use_route::<Route>();

    rsx! {
        div {
            style: "
                min-height: 100vh;
                background-color: #f5f5f5;
            ",
            
            // Header
            header {
                style: "
                    background: white;
                    border-bottom: 1px solid #ecf0f1;
                    padding: 1rem 2rem;
                    box-shadow: 0 2px 4px rgba(0,0,0,0.1);
                ",
                
                div {
                    style: "
                        display: flex;
                        justify-content: space-between;
                        align-items: center;
                        max-width: 1200px;
                        margin: 0 auto;
                    ",
                    
                    div {
                        style: "
                            display: flex;
                            align-items: center;
                            gap: 1rem;
                        ",
                        
                        h1 {
                            style: "
                                color: #2c3e50;
                                margin: 0;
                                font-size: 1.5rem;
                                font-weight: 600;
                            ",
                            "mTLS Proxy"
                        }
                        
                        span {
                            style: "
                                background-color: #27ae60;
                                color: white;
                                padding: 0.25rem 0.5rem;
                                border-radius: 4px;
                                font-size: 0.75rem;
                                font-weight: 500;
                            ",
                            "v1.0.0"
                        }
                    }
                    
                    div {
                        style: "
                            display: flex;
                            align-items: center;
                            gap: 1rem;
                        ",
                        
                        button {
                            style: "
                                padding: 0.5rem 1rem;
                                background-color: #3498db;
                                color: white;
                                border: none;
                                border-radius: 4px;
                                cursor: pointer;
                                font-size: 0.875rem;
                            ",
                            onclick: move |_| {
                                // TODO: Implement restart proxy
                            },
                            "Restart Proxy"
                        }
                        
                        button {
                            style: "
                                padding: 0.5rem 1rem;
                                background-color: #f8f9fa;
                                color: #2c3e50;
                                border: 1px solid #dee2e6;
                                border-radius: 4px;
                                cursor: pointer;
                                font-size: 0.875rem;
                            ",
                            onclick: move |_| {
                                // TODO: Implement settings
                            },
                            "Settings"
                        }
                    }
                }
            }
            
            // Navigation
            nav {
                style: "
                    background: white;
                    border-bottom: 1px solid #ecf0f1;
                    padding: 0 2rem;
                ",
                
                div {
                    style: "
                        max-width: 1200px;
                        margin: 0 auto;
                    ",
                    
                    ul {
                        style: "
                            display: flex;
                            list-style: none;
                            margin: 0;
                            padding: 0;
                            gap: 0;
                        ",
                        
                        li {
                            style: "
                                margin: 0;
                            ",
                            
                            button {
                                style: "
                                    padding: 1rem 1.5rem;
                                    background: none;
                                    border: none;
                                    color: #7f8c8d;
                                    cursor: pointer;
                                    font-size: 1rem;
                                    font-weight: 500;
                                    border-bottom: 3px solid transparent;
                                    transition: all 0.2s;
                                ",
                                onclick: move |_| {
                                    navigator.push(Route::Dashboard {});
                                },
                                "Dashboard"
                            }
                        }
                        
                        li {
                            style: "
                                margin: 0;
                            ",
                            
                            button {
                                style: "
                                    padding: 1rem 1.5rem;
                                    background: none;
                                    border: none;
                                    color: #7f8c8d;
                                    cursor: pointer;
                                    font-size: 1rem;
                                    font-weight: 500;
                                    border-bottom: 3px solid transparent;
                                    transition: all 0.2s;
                                ",
                                onclick: move |_| {
                                    navigator.push(Route::Logs {});
                                },
                                "Logs"
                            }
                        }
                        
                        li {
                            style: "
                                margin: 0;
                            ",
                            
                            button {
                                style: "
                                    padding: 1rem 1.5rem;
                                    background: none;
                                    border: none;
                                    color: #7f8c8d;
                                    cursor: pointer;
                                    font-size: 1rem;
                                    font-weight: 500;
                                    border-bottom: 3px solid transparent;
                                    transition: all 0.2s;
                                ",
                                onclick: move |_| {
                                    navigator.push(Route::Config {});
                                },
                                "Configuration"
                            }
                        }
                    }
                }
            }
            
            // Main content area
            main {
                style: "
                    flex: 1;
                    padding: 0;
                ",
                
                crate::app::Outlet {}
            }
        }
    }
}
