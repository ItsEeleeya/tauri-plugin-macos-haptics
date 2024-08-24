import { Router } from "@solidjs/router";
import { FileRoutes } from "@solidjs/start/router";

import "./styles.css";

export default function App() {
  return (
    <Router>
      <FileRoutes />
    </Router>
  );
}
