resource "aws_route53domains_registered_domain" "qqself" {
  domain_name = "qqself.com"
  dynamic "name_server" {
    for_each = aws_route53_zone.qqself.name_servers
    content {
      name = name_server.value
    }
  }
}

resource "aws_route53_zone" "qqself" {
  name = "qqself.com"
}

resource "aws_route53_record" "www" {
  zone_id = aws_route53_zone.qqself.zone_id
  name    = "www.qqself.com"
  type    = "A"

  alias {
    name                   = var.www-destination-name
    zone_id                = var.www-destination-hosted_zone_id
    evaluate_target_health = false
  }
}

resource "aws_route53_record" "app" {
  zone_id = aws_route53_zone.qqself.zone_id
  name    = "app.qqself.com"
  type    = "A"

  alias {
    name                   = var.app-destination-name
    zone_id                = var.app-destination-hosted_zone_id
    evaluate_target_health = false
  }
}

resource "aws_route53_record" "api2" {
  zone_id = aws_route53_zone.qqself.zone_id
  name    = "api2.qqself.com"
  type    = "A"

  alias {
    name                   = var.api2-destination-name
    zone_id                = var.api2-destination-hosted_zone_id
    evaluate_target_health = false
  }
}

resource "aws_route53_record" "naked" {
  zone_id = aws_route53_zone.qqself.zone_id
  name    = "qqself.com"
  type    = "A"

  alias {
    name                   = aws_route53_record.www.name
    zone_id                = aws_route53_record.www.zone_id
    evaluate_target_health = false
  }
}

resource "aws_route53_record" "api" {
  zone_id = aws_route53_zone.qqself.zone_id
  name    = "api.qqself.com"
  type    = "CNAME"
  ttl     = 300
  records = ["nu2qwbtubz.us-east-1.awsapprunner.com"]
}

resource "aws_route53_record" "email" {
  zone_id = aws_route53_zone.qqself.zone_id
  name    = ""
  type    = "MX"
  ttl     = 300
  records = [
    "1 ASPMX.L.GOOGLE.COM.",
    "5 ALT1.ASPMX.L.GOOGLE.COM.",
    "5 ALT2.ASPMX.L.GOOGLE.COM.",
    "10 ALT3.ASPMX.L.GOOGLE.COM.",
    "10 ALT4.ASPMX.L.GOOGLE.COM.",
  ]
}

resource "aws_acm_certificate" "certificate" {
  domain_name               = "*.qqself.com"
  validation_method         = "EMAIL"
  subject_alternative_names = ["qqself.com"]
}

output "certificate_arn" {
  value = aws_acm_certificate.certificate.arn
}
