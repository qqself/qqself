locals {
  origin = "s3-origin"
  tags = {
    owner = "site-www"
  }
}

resource "aws_s3_bucket" "sources" {
  bucket = "qqself-${var.name-suffix}-site-www"
  tags   = local.tags
}

resource "aws_s3_bucket_website_configuration" "sources" {
  bucket = aws_s3_bucket.sources.id

  index_document {
    suffix = "index.html"
  }
}

resource "aws_cloudfront_origin_access_identity" "site-www" {}

resource "aws_s3_bucket_policy" "cloudfront_access" {
  bucket = aws_s3_bucket.sources.id
  policy = data.aws_iam_policy_document.allow_access_cloudfront.json
}

data "aws_iam_policy_document" "allow_access_cloudfront" {
  statement {
    principals {
      type        = "AWS"
      identifiers = [aws_cloudfront_origin_access_identity.site-www.iam_arn]
    }
    actions = [
      "s3:GetObject",
      "s3:ListBucket",
    ]
    resources = [
      aws_s3_bucket.sources.arn,
      "${aws_s3_bucket.sources.arn}/*",
    ]
  }
}

resource "aws_cloudfront_distribution" "s3_distribution" {
  default_root_object = "index.html"
  enabled             = true
  http_version        = "http3"
  is_ipv6_enabled     = true
  tags                = local.tags

  origin {
    domain_name = aws_s3_bucket.sources.bucket_regional_domain_name
    origin_id   = local.origin
    s3_origin_config {
      origin_access_identity = aws_cloudfront_origin_access_identity.site-www.cloudfront_access_identity_path
    }
  }

  default_cache_behavior {
    allowed_methods        = ["HEAD", "GET"]
    cached_methods         = ["HEAD", "GET"]
    compress               = true
    default_ttl            = 3600
    max_ttl                = 86400
    min_ttl                = 0
    target_origin_id       = local.origin
    viewer_protocol_policy = "redirect-to-https"

    forwarded_values {
      query_string = false

      cookies {
        forward = "none"
      }
    }
  }

  restrictions {
    geo_restriction {
      restriction_type = "none"
    }
  }

  viewer_certificate {
    cloudfront_default_certificate = true
  }
}

output "cloudfront_domain" {
  value = aws_cloudfront_distribution.s3_distribution.domain_name
}
