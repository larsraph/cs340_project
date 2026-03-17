import { Title } from "@solidjs/meta";
import DbFetchTable from "~/components/DbFetchTable";

export default function Clubs() {
  return (
    <main>
      <Title>Clubs</Title>
      <h1>Clubs</h1>
      <DbFetchTable table="Clubs" />
    </main>
  );
}
