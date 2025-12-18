const server = Bun.serve({
  routes: {
    "/": new Response("Ok"),
  },

  fetch(req) {
    return new Response("Not found", { status: 404 });
  },
});

console.log(`Server running on ${server.url}`);
