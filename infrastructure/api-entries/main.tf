locals {
  function_name = "entries-health"
}

data "aws_iam_policy_document" "minimum_access" {
  statement {
    effect = "Allow"
    principals {
      type        = "Service"
      identifiers = ["lambda.amazonaws.com"]
    }
    actions = ["sts:AssumeRole"]
  }
}

resource "aws_iam_role" "minimum_access" {
  name               = "minimum_access"
  assume_role_policy = data.aws_iam_policy_document.minimum_access.json
}

data "aws_iam_policy_document" "logging" {
  statement {
    effect = "Allow"
    actions = [
      "logs:CreateLogGroup",
      "logs:CreateLogStream",
      "logs:PutLogEvents",
    ]
    resources = ["${aws_cloudwatch_log_group.entries_health.arn}:*"]
  }
}

resource "aws_iam_policy" "lambda_logging" {
  name   = "lambda_logging"
  path   = "/"
  policy = data.aws_iam_policy_document.logging.json
}

resource "aws_iam_role_policy_attachment" "lambda_logs" {
  role       = aws_iam_role.minimum_access.name
  policy_arn = aws_iam_policy.lambda_logging.arn
}

resource "aws_cloudwatch_log_group" "entries_health" {
  name              = "/aws/lambda/${local.function_name}"
  retention_in_days = 7
}

# HACK: We can't define empty Lambda without a file, so here some empty one
# Actual Lambda will be created and deployed through the Github Actions
data "archive_file" "empty" {
  type        = "zip"
  output_path = "${path.module}/empty-lambda.zip"
  source {
    content  = "bootstrap"
    filename = "bootstrap"
  }
}

resource "aws_lambda_function" "entries_health" {
  filename         = data.archive_file.empty.output_path
  function_name    = local.function_name
  handler          = "bootstrap"
  role             = aws_iam_role.minimum_access.arn
  runtime          = "provided.al2"
  source_code_hash = data.archive_file.empty.output_base64sha256
  architectures    = ["arm64"]
  lifecycle {
    ignore_changes = [
      source_code_hash # We deploy new version via CI, so it can be ignored
    ]
  }
}

resource "aws_apigatewayv2_api" "entries-api" {
  name                         = "entries-api"
  protocol_type                = "HTTP"
  disable_execute_api_endpoint = true
  route_key                    = "GET /health"
  target                       = aws_lambda_function.entries_health.arn
}

resource "aws_lambda_permission" "api_gateway_invoke_permission" {
  action        = "lambda:InvokeFunction"
  function_name = aws_lambda_function.entries_health.arn
  principal     = "apigateway.amazonaws.com"
  source_arn    = "${aws_apigatewayv2_api.entries-api.execution_arn}/*/*/health"
}

resource "aws_apigatewayv2_domain_name" "api2" {
  domain_name = "api2.qqself.com"
  domain_name_configuration {
    certificate_arn = var.certificate-arn
    endpoint_type   = "REGIONAL"
    security_policy = "TLS_1_2"
  }
}

resource "aws_apigatewayv2_api_mapping" "domain_mapping" {
  api_id      = aws_apigatewayv2_api.entries-api.id
  domain_name = aws_apigatewayv2_domain_name.api2.id
  stage       = "$default"
}

output "api_gateway_domain" {
  value = aws_apigatewayv2_domain_name.api2.domain_name_configuration[0].target_domain_name
}

output "api_gateway_zone_id" {
  value = aws_apigatewayv2_domain_name.api2.domain_name_configuration[0].hosted_zone_id
}
# aws lambda update-function-code --function-name entries-health --zip-file fileb://./target/lambda/basic/bootstrap.zip
