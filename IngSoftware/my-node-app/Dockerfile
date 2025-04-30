# Use the official Node.js LTS image as the base image
FROM node:lts

# Set the working directory inside the container
WORKDIR /usr/src/app

# Copy package.json and package-lock.json to the working directory
COPY package*.json ./

# Install Node.js dependencies
RUN npm install

# Copy all project files to the working directory (src/ eccetera..)
COPY . .

# Expose port 3000 to allow external access
EXPOSE 3000

# Define the command to run the application
CMD ["node", "app.js"]

# docker build -t my-app-image .
# docker run -d -p 3000:3000 -v /Users/nicolotermine/zMellow/GitHub-Poli/Polito/IngSoftware/my-node-app:/app my-app-image
