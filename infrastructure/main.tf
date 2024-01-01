provider "aws" {
  region = var.aws_region
  default_tags {
    tags = {
      environment = "qqself-${var.name-suffix}"
    }
  }
}

module "site-www" {
  source      = "./site-www"
  name-suffix = var.name-suffix
}

output "cloudfront_domain" {
  value = module.site-www.cloudfront_domain
}
