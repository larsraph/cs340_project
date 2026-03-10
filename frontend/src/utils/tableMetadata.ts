// Table metadata for primary keys and foreign keys
export type ForeignKeyMetadata = {
  column: string;
  referencesTable: string;
  referencesColumn: string;
};

export type ColumnMetadata = {
  nullable?: boolean;
  autoIncrement?: boolean;
  dataType?: 'date' | 'datetime';
};

type TableMetadata = {
  primaryKeys: string[];
  foreignKeys?: ForeignKeyMetadata[];
  columns?: Record<string, ColumnMetadata>;
};

export const TABLE_METADATA: Record<string, TableMetadata> = {
  Addresses: {
    primaryKeys: ['address_id'],
    columns: {
      address_id: { autoIncrement: true },
      address_ln2: { nullable: true },
      city: { nullable: true },
      state: { nullable: true },
    },
  },
  Clubs: {
    primaryKeys: ['club_id'],
    columns: {
      club_id: { autoIncrement: true },
      date_created: { dataType: 'date' },
    },
  },
  Roles: {
    primaryKeys: ['role_id'],
    columns: {
      role_id: { autoIncrement: true },
    },
  },
  People: {
    primaryKeys: ['person_id', 'address_id'],
    foreignKeys: [
      { column: 'address_id', referencesTable: 'Addresses', referencesColumn: 'address_id' },
    ],
    columns: {
      person_id: { autoIncrement: true },
      date_of_birth: { dataType: 'date' },
    },
  },
  Events: {
    primaryKeys: ['event_id'],
    foreignKeys: [
      { column: 'club_id', referencesTable: 'Clubs', referencesColumn: 'club_id' },
      { column: 'organizer_id', referencesTable: 'People', referencesColumn: 'person_id' },
    ],
    columns: {
      event_id: { autoIncrement: true },
      time_start: { dataType: 'datetime' },
      time_end: { dataType: 'datetime' },
    },
  },
  Membership: {
    primaryKeys: ['person_id', 'role_id', 'club_id'],
    foreignKeys: [
      { column: 'person_id', referencesTable: 'People', referencesColumn: 'person_id' },
      { column: 'role_id', referencesTable: 'Roles', referencesColumn: 'role_id' },
      { column: 'club_id', referencesTable: 'Clubs', referencesColumn: 'club_id' },
    ],
  },
  PhysicalEvents: {
    primaryKeys: ['event_id', 'address_id'],
    foreignKeys: [
      { column: 'event_id', referencesTable: 'Events', referencesColumn: 'event_id' },
      { column: 'address_id', referencesTable: 'Addresses', referencesColumn: 'address_id' },
    ],
  },
  VirtualEvents: {
    primaryKeys: ['event_id'],
    foreignKeys: [
      { column: 'event_id', referencesTable: 'Events', referencesColumn: 'event_id' },
    ],
  },
};

export const getPrimaryKeys = (tableName: string): string[] => {
  return TABLE_METADATA[tableName]?.primaryKeys || [];
};

export const getForeignKeys = (tableName: string): ForeignKeyMetadata[] => {
  return TABLE_METADATA[tableName]?.foreignKeys || [];
};

export const getForeignKeyByColumn = (tableName: string, column: string): ForeignKeyMetadata | undefined => {
  return getForeignKeys(tableName).find((foreignKey) => foreignKey.column === column);
};

export const getColumnMetadata = (tableName: string, column: string): ColumnMetadata => {
  return TABLE_METADATA[tableName]?.columns?.[column] || {};
};

export const isNullable = (tableName: string, column: string): boolean => {
  return getColumnMetadata(tableName, column).nullable || false;
};

export const isAutoIncrement = (tableName: string, column: string): boolean => {
  return getColumnMetadata(tableName, column).autoIncrement || false;
};

export const getColumnDataType = (tableName: string, column: string): 'date' | 'datetime' | undefined => {
  return getColumnMetadata(tableName, column).dataType;
};
