# -- build UI --
FROM node:23-alpine AS build 

WORKDIR /app 

COPY package.json package-lock.json .
RUN npm ci 

COPY . .
RUN npm run build

# -- Nginx  --
FROM nginx:stable-alpine AS prod
COPY --from=build /app/dist /usr/share/nginx/html
COPY ./nginx/nginx.prod.conf /etc/nginx/templates/nginx.conf.template
COPY ./nginx/nginx.conf /etc/nginx/nginx.conf


