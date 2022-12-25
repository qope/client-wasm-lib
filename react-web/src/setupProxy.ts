import { createProxyMiddleware } from "http-proxy-middleware";

module.exports = function (app: any) {
  app.use(function (req: any, res: any, next: any) {
    res.setHeader("Cross-Origin-Opener-Policy", "same-origin");
    res.setHeader("Cross-Origin-Embedder-Policy", "require-corp");
    next();
  });
};
