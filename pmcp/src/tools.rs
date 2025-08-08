use crate::{Result, Tool};
use serde_json::json;

pub fn calculator_tool() -> Tool {
    Tool {
        name: "calculator".to_string(),
        description: "Perform arithmetic calculations".to_string(),
        input_schema: json!({
            "type": "object",
            "properties": {
                "operation": {
                    "type": "string",
                    "enum": ["add", "subtract", "multiply", "divide"]
                },
                "a": {
                    "type": "number"
                },
                "b": {
                    "type": "number"
                }
            },
            "required": ["operation", "a", "b"]
        }),
    }
}

pub fn analyze_complexity_tool() -> Tool {
    Tool {
        name: "analyze_complexity".to_string(),
        description: "Analyze code complexity metrics".to_string(),
        input_schema: json!({
            "type": "object",
            "properties": {
                "code": {
                    "type": "string"
                },
                "language": {
                    "type": "string",
                    "enum": ["rust", "python", "javascript", "typescript"]
                }
            },
            "required": ["code", "language"]
        }),
    }
}

pub fn extract_files_tool() -> Tool {
    Tool {
        name: "extract_files".to_string(),
        description: "Extract files from a directory".to_string(),
        input_schema: json!({
            "type": "object",
            "properties": {
                "path": {
                    "type": "string"
                },
                "pattern": {
                    "type": "string"
                },
                "recursive": {
                    "type": "boolean"
                }
            },
            "required": ["path"]
        }),
    }
}

pub fn deep_analysis_tool() -> Tool {
    Tool {
        name: "deep_analysis".to_string(),
        description: "Perform deep code analysis".to_string(),
        input_schema: json!({
            "type": "object",
            "properties": {
                "project_path": {
                    "type": "string"
                },
                "analysis_type": {
                    "type": "string",
                    "enum": ["security", "performance", "quality", "all"]
                }
            },
            "required": ["project_path", "analysis_type"]
        }),
    }
}