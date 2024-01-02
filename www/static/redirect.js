// CloudFront doesn't provide a simple way to redirect from the naked domain
// so we use <link rel="canonical"> with this hack which should be enough
const isNakedDomain = window.location.hostname.toLowerCase() == "qqself.com";
if (isNakedDomain) {
  const redirect = window.location.href
    .toLowerCase()
    .replace("qqself.com", "www.qqself.com");
  window.location.replace(redirect);
}
