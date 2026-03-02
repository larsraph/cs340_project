/*
 * Authored and maintained by GitHub Copilot (GPT-5.3-Codex)
 * Date: March 2, 2026
 */

import { createResource, For, Suspense, createSignal, Setter, createEffect, createMemo, onCleanup, onMount } from 'solid-js';
import { getPrimaryKeys } from '~/utils/tableMetadata';
import "./DbFetchTable.css";


type DbFetchTableProps = {
  table: string;
};

const formatColumnHeader = (key: string): string => {
  return key
    .split('_')
    .map((word) => word.charAt(0).toUpperCase() + word.slice(1))
    .join(' ');
};

// Type detection helper
const detectType = (value: any): 'number' | 'boolean' | 'string' | 'date' => {
  if (typeof value === 'number') return 'number';
  if (typeof value === 'boolean') return 'boolean';
  if (value instanceof Date || (typeof value === 'string' && /^\d{4}-\d{2}-\d{2}/.test(value))) return 'date';
  return 'string';
};

// Casting helper
const castValue = (value: string, type: 'number' | 'boolean' | 'string' | 'date'): any => {
  if (!value && value !== '0') return null;
  
  switch (type) {
    case 'number':
      const num = parseFloat(value);
      return isNaN(num) ? value : num;
    case 'boolean':
      return value === '1' || value === 'true';
    case 'date':
      return value;
    default:
      return value;
  }
};

type DataRowProps = {
  rowIndex: number;
  row: Record<string, any>;
  columns: string[];
  columnTypes: Record<string, 'number' | 'boolean' | 'string' | 'date'>;
  primaryKeys: string[];
  table: string;
  editingId: number | null;
  setEditingId: Setter<number | null>;
  onDelete: (index: number) => Promise<void>;
  onUpdate: (index: number, data: Record<string, any>) => Promise<void>;
};

const DataRow = (props: DataRowProps) => {
  const [editData, setEditData] = createSignal<Record<string, any>>({});
  const [error, setError] = createSignal<string>('');
  const [loading, setLoading] = createSignal<boolean>(false);
  const isEditing = createMemo(() => props.editingId === props.rowIndex);

  const handleEditClick = () => {
    if (isEditing()) {
      props.setEditingId(null);
    } else {
      const data: Record<string, any> = {};
      props.columns.forEach(col => {
        data[col] = props.row[col];
      });
      setEditData(data);
      props.setEditingId(props.rowIndex);
    }
  };

  const handleInputChange = (column: string, value: string) => {
    setEditData(prev => ({ ...prev, [column]: value }));
  };

  const handleDelete = async () => {
    setLoading(true);
    setError('');
    try {
      await props.onDelete(props.rowIndex);
    } catch (err) {
      setError(String(err));
    } finally {
      setLoading(false);
    }
  };

  const handleSubmit = async () => {
    setLoading(true);
    setError('');
    try {
      const castData: Record<string, any> = {};
      props.columns.forEach(col => {
        castData[col] = castValue(editData()[col] || '', props.columnTypes[col]);
      });
      await props.onUpdate(props.rowIndex, castData);
      props.setEditingId(null);
    } catch (err) {
      setError(String(err));
    } finally {
      setLoading(false);
    }
  };

  return (
    <>
      <tr>
        <For each={props.columns}>
          {(column) => (
            <td>
              {isEditing() ? (
                <input
                  type="text"
                  value={editData()[column] || ''}
                  onInput={(e) => handleInputChange(column, e.currentTarget.value)}
                  disabled={loading()}
                />
              ) : (
                String(props.row[column])
              )}
            </td>
          )}
        </For>
        <td class="trash">
          <button onClick={handleDelete} disabled={loading()}>🗑️</button>
          {isEditing() ? (
            <button onClick={handleSubmit} disabled={loading()}>✓</button>
          ) : (
            <button onClick={handleEditClick} disabled={loading()}>✏️</button>
          )}
        </td>
      </tr>
      {error() && (
        <tr>
          <td colSpan={props.columns.length + 1} style={{ color: 'red', padding: '10px' }}>
            Error: {error()}
          </td>
        </tr>
      )}
    </>
  );
};

type FormRowProps = {
  columns: string[];
  columnTypes: Record<string, 'number' | 'boolean' | 'string' | 'date'>;
  onAddRow: (row: Record<string, any>) => Promise<void>;
};

const FormRow = (props: FormRowProps) => {
  const [formData, setFormData] = createSignal<Record<string, string>>({});
  const [error, setError] = createSignal<string>('');
  const [loading, setLoading] = createSignal<boolean>(false);

  const handleInputChange = (column: string, value: string) => {
    setFormData(prev => ({ ...prev, [column]: value }));
  };

  const handleSubmit = async () => {
    setLoading(true);
    setError('');
    try {
      const castData: Record<string, any> = {};
      props.columns.forEach(col => {
        castData[col] = castValue(formData()[col] || '', props.columnTypes[col]);
      });
      await props.onAddRow(castData);
      setFormData({});
    } catch (err) {
      setError(String(err));
    } finally {
      setLoading(false);
    }
  };

  return (
    <>
      <tr class="form-row">
        <For each={props.columns}>
          {(column) => (
            <td>
              <input
                type="text"
                placeholder={formatColumnHeader(column)}
                value={formData()[column] || ''}
                onInput={(e) => handleInputChange(column, e.currentTarget.value)}
                disabled={loading()}
              />
            </td>
          )}
        </For>
        <td class="trash">
          <button onClick={handleSubmit} disabled={loading()}>✓</button>
        </td>
      </tr>
      {error() && (
        <tr>
          <td colSpan={props.columns.length + 1} style={{ color: 'red', padding: '10px' }}>
            Error: {error()}
          </td>
        </tr>
      )}
    </>
  );
};

const DbFetchTable = (props: DbFetchTableProps) => {
  const [editingId, setEditingId] = createSignal<number | null>(null);
  const [tableData, setTableData] = createSignal<Record<string, any>[]>([]);
  const [columns, setColumns] = createSignal<string[]>([]);
  const [columnTypes, setColumnTypes] = createSignal<Record<string, 'number' | 'boolean' | 'string' | 'date'>>({});
  const [refetchCount, setRefetchCount] = createSignal<number>(0);
  const primaryKeys = getPrimaryKeys(props.table);

  const getApiUrl = (table: string): string => {
    if (import.meta.env.SSR) {
      const port = process.env.PORT || process.env.VITE_PORT || '3000';
      return `http://localhost:${port}/api/${table}`;
    }
    return `/api/${table}`;
  };

  const [data] = createResource(
    () => ({ table: props.table, refetch: refetchCount() }),
    async ({ table }) => {
      const response = await fetch(getApiUrl(table));
      if (!response.ok) {
        throw new Error(`Failed to fetch table data: ${response.statusText}`);
      }
      return response.json();
    }
  );

  createEffect(() => {
    if (data()) {
      setTableData([...data()]);
      if (data().length > 0) {
        const cols = Object.keys(data()[0]);
        setColumns(cols);
        
        // Detect types from first row
        const types: Record<string, 'number' | 'boolean' | 'string' | 'date'> = {};
        cols.forEach(col => {
          types[col] = detectType(data()[0][col]);
        });
        setColumnTypes(types);
      }
    }
  });

  onMount(() => {
    const eventSource = new EventSource('/api/stream');

    const handleDbUpdate = (event: MessageEvent) => {
      try {
        const payload = JSON.parse(event.data) as { table?: string };
        if (payload.table === props.table || payload.table === '*') {
          setRefetchCount((count) => count + 1);
        }
      } catch (error) {
        console.error('Failed to parse db-update event:', error);
      }
    };

    eventSource.addEventListener('db-update', handleDbUpdate);

    onCleanup(() => {
      eventSource.removeEventListener('db-update', handleDbUpdate);
      eventSource.close();
    });
  });

  const handleDeleteRow = async (index: number) => {
    const row = tableData()[index];
    const where: Record<string, any> = {};
    
    primaryKeys.forEach(pk => {
      where[pk] = row[pk];
    });

    try {
      const response = await fetch(getApiUrl(props.table), {
        method: 'DELETE',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ where }),
      });

      if (!response.ok) {
        const error = await response.json();
        throw new Error(error.error || 'Failed to delete row');
      }

      const currentEditingId = editingId();
      if (currentEditingId !== null) {
        if (currentEditingId === index) {
          setEditingId(null);
        } else if (currentEditingId > index) {
          setEditingId(currentEditingId - 1);
        }
      }

      // Refetch data
      setRefetchCount(c => c + 1);
    } catch (err) {
      throw err;
    }
  };

  const handleUpdateRow = async (index: number, newData: Record<string, any>) => {
    const oldRow = tableData()[index];
    const where: Record<string, any> = {};
    
    primaryKeys.forEach(pk => {
      where[pk] = oldRow[pk];
    });

    try {
      const response = await fetch(getApiUrl(props.table), {
        method: 'PUT',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ data: newData, where }),
      });

      if (!response.ok) {
        const error = await response.json();
        throw new Error(error.error || 'Failed to update row');
      }

      // Refetch data
      setRefetchCount(c => c + 1);
    } catch (err) {
      throw err;
    }
  };

  const handleAddRow = async (newRow: Record<string, any>) => {
    try {
      const response = await fetch(getApiUrl(props.table), {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ data: newRow }),
      });

      if (!response.ok) {
        const error = await response.json();
        throw new Error(error.error || 'Failed to create row');
      }

      // Refetch data
      setRefetchCount(c => c + 1);
    } catch (err) {
      throw err;
    }
  };

  return (
    <div class="table-container">
      <Suspense fallback={<p>Loading data...</p>}>
        {columns().length > 0 ? (
          <table class="db-table">
            <thead>
              <tr>
                <For each={columns()}>
                  {(key) => <th>{formatColumnHeader(key)}</th>}
                </For>
              </tr>
            </thead>
            <tbody>
              <For each={tableData()}>
                {(row, index) => (
                  <DataRow
                    rowIndex={index()}
                    row={row}
                    columns={columns()}
                    columnTypes={columnTypes()}
                    primaryKeys={primaryKeys}
                    table={props.table}
                    editingId={editingId()}
                    setEditingId={setEditingId}
                    onDelete={handleDeleteRow}
                    onUpdate={handleUpdateRow}
                  />
                )}
              </For>
              <FormRow 
                columns={columns()} 
                columnTypes={columnTypes()}
                onAddRow={handleAddRow} 
              />
            </tbody>
          </table>
        ) : (
          <p>No data available</p>
        )}
      </Suspense>
    </div>
  );
}

export default DbFetchTable;