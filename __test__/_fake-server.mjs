// Minimal stand-in for a HyperSync server, bound to loopback, so client tests
// can inspect exactly what the binding sends without network access or a token.
import http from "node:http";

export const RATE_LIMIT_HEADERS = {
  "x-ratelimit-limit": "50, 50;w=60",
  "x-ratelimit-remaining": "0",
  "x-ratelimit-reset": "41",
  "x-ratelimit-cost": "10",
};

// Answers /height and /chain_id, and replies 429 to queries so the request
// body can be captured without having to produce an Arrow response.
export async function startFakeServer({ rateLimitHeaders = RATE_LIMIT_HEADERS } = {}) {
  const requests = [];
  const server = http.createServer((req, res) => {
    let body = "";
    req.on("data", (chunk) => (body += chunk));
    req.on("end", () => {
      requests.push({ method: req.method, url: req.url, headers: req.headers, body });
      res.setHeader("content-type", "application/json");
      if (req.url === "/height") return res.end(JSON.stringify({ height: 42 }));
      if (req.url === "/chain_id") return res.end(JSON.stringify({ chain_id: 7 }));
      res.statusCode = 429;
      for (const [k, v] of Object.entries(rateLimitHeaders)) res.setHeader(k, v);
      res.end("rate limited");
    });
  });
  await new Promise((resolve) => server.listen(0, "127.0.0.1", resolve));
  return {
    url: `http://127.0.0.1:${server.address().port}`,
    requests,
    queries: () => requests.filter((r) => r.url.startsWith("/query")).map((r) => JSON.parse(r.body)),
    close: () => new Promise((resolve) => server.close(resolve)),
  };
}

export const clientConfig = (url, extra = {}) => ({
  url,
  apiToken: "tok-123",
  maxNumRetries: 0,
  proactiveRateLimitSleep: false,
  ...extra,
});
