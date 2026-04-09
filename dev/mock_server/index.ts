const server = Bun.serve({
  routes: {
    "/": Response.json({ msg: "Ok" }),
    "/user": {
      GET: () => {
        return new Response("Get user");
      },
    },
  },

  fetch(req) {
    return new Response("Not found", { status: 404 });
  },
});

console.log(`Server running on ${server.url}`);
