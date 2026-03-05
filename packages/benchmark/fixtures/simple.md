# Getting Started with Node.js

Node.js is an open-source, cross-platform JavaScript runtime environment that executes
JavaScript code outside of a web browser. It allows developers to use JavaScript to write
server-side scripting, producing dynamic web page content before the page is sent to the
user's web browser.

## Installation

You can install Node.js from the [official website](https://nodejs.org/) or by using a
version manager like **nvm** (Node Version Manager). Using a version manager is recommended
because it allows you to switch between different versions of Node.js easily.

To verify your installation, open a terminal and run:

```bash
node --version
npm --version
```

If both commands return version numbers, you are ready to go.

## Your First Application

Create a new directory for your project and initialize it:

1. Open your terminal
2. Create a project directory
3. Navigate into it
4. Run the initialization command

Inside your project folder, create a file called `index.js` and add the following code:

```js
const http = require("http");

const server = http.createServer((req, res) => {
  res.writeHead(200, { "Content-Type": "text/plain" });
  res.end("Hello, World!\n");
});

server.listen(3000, () => {
  console.log("Server running at http://localhost:3000/");
});
```

Start your server by running `node index.js` in the terminal. Then open your browser and
navigate to [http://localhost:3000](http://localhost:3000) to see the result.

## Core Concepts

Node.js uses an **event-driven**, **non-blocking I/O** model that makes it lightweight and
efficient. Here are some core concepts you should understand:

- **Event Loop** - The mechanism that allows Node.js to perform non-blocking operations
- **Callbacks** - Functions passed as arguments to other functions, executed after completion
- **Promises** - Objects representing the eventual completion or failure of an async operation
- **Async/Await** - Syntactic sugar built on top of promises for cleaner async code
- **Streams** - Collections of data that might not be available all at once
- **Buffers** - Temporary storage for binary data

### The Event Loop

The event loop is what allows Node.js to perform non-blocking I/O operations despite the
fact that JavaScript is *single-threaded*. It offloads operations to the system kernel
whenever possible.

When Node.js starts, it initializes the event loop, processes the provided input script,
and then begins processing the event loop. Each iteration of the event loop is called a
"tick."

### Modules

Node.js has a built-in module system. Every file in Node.js is treated as a separate module.
You can export values from a module using `module.exports` and import them in another file
using `require()`.

Common built-in modules include:

- `fs` - File system operations
- `path` - File path utilities
- `http` - HTTP server and client
- `os` - Operating system information
- `crypto` - Cryptographic functions
- `events` - Event emitter pattern

## Package Management

npm (Node Package Manager) is the default package manager for Node.js. It provides access
to hundreds of thousands of reusable packages. You can install packages locally for a
specific project or globally for system-wide use.

To install a package locally:

```bash
npm install express
```

To install a package globally:

```bash
npm install -g nodemon
```

Your project's dependencies are tracked in the `package.json` file, which also contains
metadata about your project such as its name, version, and scripts.

## Next Steps

Once you are comfortable with the basics, consider exploring these topics:

- Building REST APIs with [Express](https://expressjs.com/)
- Working with databases using *Mongoose* or *Sequelize*
- Real-time communication with **Socket.io**
- Testing with frameworks like Jest or Mocha
- Deploying applications to cloud platforms

For more information, check the [Node.js documentation](https://nodejs.org/en/docs/) and
the [npm registry](https://www.npmjs.com/) for packages that can help accelerate your
development workflow.
