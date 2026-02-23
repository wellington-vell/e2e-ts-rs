import { defineConfig } from "@hey-api/openapi-ts";
import "dotenv/config";

export default defineConfig({
  input: { path: `${process.env.VITE_SERVER_URL}/openapi.json` },
  output: {
    path: "apps/web/src/lib/api",
    entryFile: false,
    postProcess: ["oxfmt"],
  },
  plugins: [
    {
      name: "zod",
      requests: false,
      responses: false,
      case: "snake_case",
    },
    {
      name: "@hey-api/sdk",
      validator: true,
      operations: "byTags",
    },
  ],
});
