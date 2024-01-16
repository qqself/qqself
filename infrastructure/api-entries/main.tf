resource "aws_apigatewayv2_api" "entries" {
  name                         = "entries"
  protocol_type                = "HTTP"
  disable_execute_api_endpoint = true
  cors_configuration {
    allow_origins = ["*"]
    allow_methods = ["GET", "POST"]
  }
}

resource "aws_apigatewayv2_stage" "v1" {
  api_id      = aws_apigatewayv2_api.entries.id
  name        = "v1"
  auto_deploy = true
  default_route_settings {
    throttling_burst_limit = 30
    throttling_rate_limit  = 10
  }
  # TODO Probably it would be helpful to store Api Gateway logs?
}

resource "aws_apigatewayv2_domain_name" "api" {
  domain_name = "api.qqself.com"
  domain_name_configuration {
    certificate_arn = var.certificate-arn
    endpoint_type   = "REGIONAL"
    security_policy = "TLS_1_2"
  }
}

resource "aws_apigatewayv2_api_mapping" "domain_mapping" {
  api_id      = aws_apigatewayv2_api.entries.id
  domain_name = aws_apigatewayv2_domain_name.api.id
  stage       = aws_apigatewayv2_stage.v1.id
}

output "api_gateway_domain" {
  value = aws_apigatewayv2_domain_name.api.domain_name_configuration[0].target_domain_name
}

output "api_gateway_zone_id" {
  value = aws_apigatewayv2_domain_name.api.domain_name_configuration[0].hosted_zone_id
}

module "lambda_health" {
  source                = "../modules/lambda"
  function_name         = "entries-health"
  http_method           = "GET"
  http_path             = "/health"
  gateway_execution_arn = aws_apigatewayv2_api.entries.execution_arn
  gateway_id            = aws_apigatewayv2_api.entries.id
}

module "lambda_set" {
  source                = "../modules/lambda"
  function_name         = "entries-set"
  http_method           = "POST"
  http_path             = "/set"
  access_dynamodb       = true
  gateway_execution_arn = aws_apigatewayv2_api.entries.execution_arn
  gateway_id            = aws_apigatewayv2_api.entries.id
}

module "lambda_find" {
  source                = "../modules/lambda"
  function_name         = "entries-find"
  http_method           = "POST"
  http_path             = "/find"
  access_dynamodb       = true
  gateway_execution_arn = aws_apigatewayv2_api.entries.execution_arn
  gateway_id            = aws_apigatewayv2_api.entries.id
}

module "lambda_delete" {
  source                = "../modules/lambda"
  function_name         = "entries-delete"
  http_method           = "POST"
  http_path             = "/delete"
  access_dynamodb       = true
  gateway_execution_arn = aws_apigatewayv2_api.entries.execution_arn
  gateway_id            = aws_apigatewayv2_api.entries.id
}

