provider "aws" {
  region = "us-east-1"
  default_tags {
    tags = {
      owner = "qqself"
    }
  }
}

module "site-www" {
  source          = "./site-www"
  certificate-arn = module.domain.certificate_arn
}

module "domain" {
  source                         = "./domain"
  www-destination-name           = module.site-www.cloudfront_domain
  www-destination-hosted_zone_id = module.site-www.cloudfron_zone_id
}
