FROM alpine:3.19.0
WORKDIR /www
RUN apk add zola
COPY www .
RUN zola build

FROM nginx:1.25.3-alpine
COPY www/nginx.conf /etc/nginx/nginx.conf
COPY --from=0 /www/public/ /app
COPY client-web/favicon.ico /app
