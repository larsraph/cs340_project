import { Title } from "@solidjs/meta";
import DbFetchTable from "~/components/DbFetchTable";

export default function Clubs() {
  return (
    <main>
      <Title>Clubs</Title>
      <h1>Clubs</h1>
      <p class="page-description">
The Clubs page stores information about student organizations, including club
ID, club names, creation dates, and whether the club is currently active.
</p>
      <DbFetchTable table="Clubs" />
    </main>
  );
}
