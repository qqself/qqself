variable "function_name" {
  description = "AWS Lambda function name"
}

variable "http_method" {
  description = "Lambda HTTP method e.g. 'GET'"
}

variable "http_path" {
  description = "Lambda HTTP path e.g. '/health'"
}

variable "gateway_execution_arn" {
  description = "AWS Gateway execution_arn"
}

variable "gateway_id" {
  description = "AWS Gateway id"
}

variable "access_dynamodb" {
  description = "Should Lambda have an access to DynamoDB table"
  type = bool
  default = false
}

