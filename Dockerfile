# Use the official nginx image as base
FROM nginx:alpine

COPY ./SynthSnakeWebDeployment/ /usr/share/nginx/html
COPY ./nginx.conf /etc/nginx/nginx.conf

EXPOSE 80
CMD ["nginx", "-g", "daemon off;"]
