# Untyped Specs
## Loading a table
Client --tname--> Server --query--> MySql --mysql_table--> Server --table--> Client
tname: `String`,
mysql_table: INTERNAL, 
table: untyped -- either AOS `Vec<HashMap<String, String>>` or SOA `HashMap<String, Vec<String>>`,

## Validating a field
Client --tname, cname, pk, value--> Server --query--> MySql --is_valid--> Server --is_valid--> Client
pk: `i32`,
tname, cname, value: `String`,

## Push
### Creating a new row
if the client validated the row
Client --tname, row--> Server --query--> MySql --error--> Server --error--> Client
tname: `String`,
row: `HashMap<String, String>`,

### Updating a field
if the client validated the field
Client --tname, cname, pk, value--> Server --query--> MySql --error--> Server --error--> Client
pk: `i32`,
tname, cname, value: `String`,

### Deleting a row
Cleint --tname, pk--> Server --query--> MySql --error--> Server --error--> Client
pk: `i32`,
tname: `String`,



# Typed Specs
## Loading a table
Client --tname--> Server --q_schema, q_table--> MySql --schema, table--> Server --un_table--> Client
tname: `String`,
schema: mysql_table, // we'll have to  
table: untyped -- either AOS `Vec<HashMap<String, String>>` or SOA `HashMap<String, Vec<String>>`,

## Validating a field
Client --tname, cname, pk, value--> Server --query--> MySql --is_valid--> Server --is_valid--> Client
pk: `i32`,
tname, cname, value: `String`,

## Push
### Creating a new row
if the client validated the row
Client --tname, row--> Server --query--> MySql --error--> Server --error--> Client
tname: `String`,
row: `HashMap<String, String>`,

### Updating a field
if the client validated the field
Client --tname, cname, pk, value--> Server --query--> MySql --error--> Server --error--> Client
pk: `i32`,
tname, cname, value: `String`,

### Deleting a row
Cleint --tname, pk--> Server --query--> MySql --error--> Server --error--> Client
pk: `i32`,
tname: `String`,