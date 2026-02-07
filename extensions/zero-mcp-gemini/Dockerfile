# Stage 1: Build the application
FROM node:20-slim AS builder

# Set the working directory
WORKDIR /app

# Enable pnpm
RUN corepack enable

# Copy dependency definition files
COPY package.json pnpm-lock.yaml pnpm-workspace.yaml tsconfig.json ./

# Install all dependencies, including devDependencies for building
RUN pnpm install --prod=false

# Copy the rest of the source code
COPY . .

# Build the TypeScript project
RUN pnpm build

# Stage 2: Create the production image
FROM node:20-slim

# Set the working directory
WORKDIR /app

# Enable pnpm
RUN corepack enable

# Copy only the production dependency definitions
COPY package.json pnpm-lock.yaml pnpm-workspace.yaml ./

# Install only production dependencies
RUN pnpm install --prod=true

# Copy the built application from the builder stage
COPY --from=builder /app/dist ./dist

# Expose the port the app runs on (if any, assuming 3000 as a placeholder)
# Update this if the application uses a different port.
EXPOSE 3000

# The command to run the application
CMD ["node", "dist/index.js"]
