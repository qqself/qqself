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

module "site-app" {
  source          = "./site-app"
  certificate-arn = module.domain.certificate_arn
}

module "domain" {
  source                          = "./domain"
  www-destination-name            = module.site-www.cloudfront_domain
  www-destination-hosted_zone_id  = module.site-www.cloudfron_zone_id
  app-destination-name            = module.site-app.cloudfront_domain
  app-destination-hosted_zone_id  = module.site-app.cloudfron_zone_id
  api2-destination-name           = module.api-entries.api_gateway_domain
  api2-destination-hosted_zone_id = module.api-entries.api_gateway_zone_id
}

module "api-entries" {
  source          = "./api-entries"
  certificate-arn = module.domain.certificate_arn
}
