import { MetaProvider, Title } from "@solidjs/meta";
import { Router } from "@solidjs/router";
import { FileRoutes } from "@solidjs/start/router";
import { Suspense } from "solid-js";
import "./app.css";

export default function App() {
  return (
    <Router
      root={props => (
        <MetaProvider>
          <Title>OSU Clubs DB</Title>
          <a href="/people">People</a>
          <a href="/membership">Membership</a>
          <a href="/clubs">Clubs</a>
          <a href="/events">Events</a>
          <a href="/addresses">Addresses</a>
          <Suspense>{props.children}</Suspense>
        </MetaProvider>
      )}
    >
      <FileRoutes />
    </Router>
  );
}
