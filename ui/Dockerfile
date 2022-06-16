FROM node:alpine
WORKDIR /app
ENV PATH /app/node_modules/.bin:$PATH

COPY package.json .
COPY package-lock.json .
RUN npm install
RUN chown -R node:node /app


#RUN chown -R node:node /app
#USER node

#RUN npm config set unsafe-perm true
#RUN mkdir -p /app/node_modules/.cache

RUN npm install react-scripts -g

COPY . .

#USER node

CMD ["npm", "run", "start"]
