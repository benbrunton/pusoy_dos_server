FROM node:8

EXPOSE 8888


ADD ./app /app

WORKDIR /app

RUN npm install forever -g
RUN npm install

CMD ["forever", "./app.js"]
