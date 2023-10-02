FROM alpine:3.18.4
WORKDIR /www
RUN apk add zola
COPY www .
RUN zola build

FROM nginx:1.25.2-alpine
COPY www/nginx.conf /etc/nginx/nginx.conf
COPY --from=0 /www/public/ /app
COPY client-web/favicon.ico /app
