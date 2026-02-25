import path from "path";

import { defineConfig } from "@hey-api/openapi-ts";
import dotenv from "dotenv";

dotenv.config({ path: path.resolve(__dirname, "../../.env") });

export default defineConfig({
  input: { path: `${process.env.VITE_SERVER_URL}/openapi.json` },
  output: {
    path: "src/lib/api",
    entryFile: false,
    postProcess: ["oxfmt"],
  },
  plugins: [
    {
      name: "zod",
      responses: false,
      case: "snake_case",
    },
    {
      name: "@hey-api/sdk",
    },
  ],
});
