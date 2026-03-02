// Table metadata for primary keys and type detection
export const TABLE_METADATA: Record<string, { primaryKeys: string[] }> = {
  Addresses: { primaryKeys: ['address_id'] },
  Clubs: { primaryKeys: ['club_id'] },
  Roles: { primaryKeys: ['role_id'] },
  People: { primaryKeys: ['person_id', 'address_id'] },
  Events: { primaryKeys: ['event_id'] },
  Membership: { primaryKeys: ['person_id', 'role_id', 'club_id'] },
  PhysicalEvents: { primaryKeys: ['event_id', 'address_id'] },
  VirtualEvents: { primaryKeys: ['event_id'] },
};

export const getPrimaryKeys = (tableName: string): string[] => {
  return TABLE_METADATA[tableName]?.primaryKeys || [];
};
