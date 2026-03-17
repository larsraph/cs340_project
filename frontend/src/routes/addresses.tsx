import { Title } from "@solidjs/meta";
import DbFetchTable from "~/components/DbFetchTable";

export default function Addresses() {
  return (
    <main>
      <Title>Addresses</Title>
      <h1>Addresses</h1>
      <p class="page-description">
        The Addresses page stores location information used by both members and
        physical events, including address ID, Country Code, Zip Code, Address, City, and State.
      </p>
      <DbFetchTable table="Addresses" />
    </main>
  );
}
