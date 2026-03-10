/*
 * Authored and maintained by GitHub Copilot (GPT-5.3-Codex)
 * Date: March 2, 2026
 */

import { createResource, For, Suspense, createSignal, Setter, createEffect, createMemo, onCleanup, onMount } from 'solid-js';
import { Portal } from 'solid-js/web';
import { getPrimaryKeys, getForeignKeys, ForeignKeyMetadata, isNullable, isAutoIncrement, getColumnDataType } from '~/utils/tableMetadata';
import "./DbFetchTable.css";
import 'flatpickr/dist/flatpickr.min.css';


type DbFetchTableProps = {
  table: string;
};

const FK_MAX_RELATIONSHIP_DEPTH = 3;
type DataType = 'number' | 'boolean' | 'string' | 'date' | 'datetime';
type RelatedRowsByTable = Record<string, Record<string, any>[]>;
type RelatedColumnsByTable = Record<string, string[]>;

const formatColumnHeader = (key: string): string => {
  return key
    .split('_')
    .map((word) => word.charAt(0).toUpperCase() + word.slice(1))
    .join(' ');
};

const isDateLikeColumnName = (column: string): boolean => {
  return /(^|_)(date|dob|birth|birthday)(_|$)/i.test(column);
};

const isDateOnlyDateTimeString = (value: string): boolean => {
  const normalized = value.trim();
  return /^\d{4}-\d{2}-\d{2}[T\s]00:00(:00(\.0+)?)?(Z|[+-]00:00)?$/i.test(normalized);
};

// Type detection helper
const detectType = (value: any, column?: string, configuredType?: 'date' | 'datetime'): DataType => {
  if (configuredType) {
    return configuredType;
  }

  const hasDateColumnHint = !!column && isDateLikeColumnName(column);

  if (typeof value === 'number') return 'number';
  if (typeof value === 'boolean') return 'boolean';
  if (value instanceof Date) return hasDateColumnHint ? 'date' : 'datetime';
  if (typeof value === 'string') {
    if (/^\d{4}-\d{2}-\d{2}$/.test(value)) return 'date';
    if (/^\d{4}-\d{2}-\d{2}T/.test(value) || /^\d{4}-\d{2}-\d{2}\s\d{2}:\d{2}/.test(value)) {
      if (hasDateColumnHint) return 'date';
      return 'datetime';
    }
    if (/^\d{4}-\d{2}-\d{2}/.test(value)) return 'date';
  }
  return 'string';
};

const toDateTimeLocalValue = (value: any): string => {
  if (value === null || value === undefined) return '';
  if (value instanceof Date) {
    if (Number.isNaN(value.getTime())) return '';
    const year = value.getFullYear();
    const month = String(value.getMonth() + 1).padStart(2, '0');
    const day = String(value.getDate()).padStart(2, '0');
    const hours = String(value.getHours()).padStart(2, '0');
    const minutes = String(value.getMinutes()).padStart(2, '0');
    return `${year}-${month}-${day}T${hours}:${minutes}`;
  }

  const raw = String(value).trim();
  const exactMatch = raw.match(/^(\d{4}-\d{2}-\d{2})[T\s](\d{2}:\d{2})(?::\d{2}(?:\.\d+)?)?$/);
  if (exactMatch) {
    return `${exactMatch[1]}T${exactMatch[2]}`;
  }

  const parsed = new Date(raw);
  if (Number.isNaN(parsed.getTime())) return '';

  const year = parsed.getFullYear();
  const month = String(parsed.getMonth() + 1).padStart(2, '0');
  const day = String(parsed.getDate()).padStart(2, '0');
  const hours = String(parsed.getHours()).padStart(2, '0');
  const minutes = String(parsed.getMinutes()).padStart(2, '0');
  return `${year}-${month}-${day}T${hours}:${minutes}`;
};

const toDateInputValue = (value: any): string => {
  if (value === null || value === undefined) return '';
  const raw = String(value).trim();
  const match = raw.match(/^(\d{4}-\d{2}-\d{2})/);
  if (match) return match[1];

  const parsed = new Date(raw);
  if (Number.isNaN(parsed.getTime())) return '';

  const year = parsed.getUTCFullYear();
  const month = String(parsed.getUTCMonth() + 1).padStart(2, '0');
  const day = String(parsed.getUTCDate()).padStart(2, '0');
  return `${year}-${month}-${day}`;
};

const formatDateForDisplay = (value: any): string => {
  const dateValue = toDateInputValue(value);
  if (!dateValue) {
    return value === null || value === undefined ? '' : String(value);
  }

  const [year, month, day] = dateValue.split('-').map(Number);
  const utcDate = new Date(Date.UTC(year, month - 1, day));
  return new Intl.DateTimeFormat(undefined, {
    year: 'numeric',
    month: 'short',
    day: 'numeric',
    timeZone: 'UTC',
  }).format(utcDate);
};

const formatDateTimeForDisplay = (value: any): string => {
  const dateTimeValue = toDateTimeLocalValue(value);
  if (!dateTimeValue) {
    return value === null || value === undefined ? '' : String(value);
  }

  const parsed = new Date(dateTimeValue);
  if (Number.isNaN(parsed.getTime())) {
    return value === null || value === undefined ? '' : String(value);
  }

  return new Intl.DateTimeFormat(undefined, {
    year: 'numeric',
    month: 'short',
    day: 'numeric',
    hour: 'numeric',
    minute: '2-digit',
  }).format(parsed);
};

const toDateTimePickerValue = (value: any): string => {
  const localValue = toDateTimeLocalValue(value);
  if (!localValue) {
    return '';
  }
  return `${localValue.replace('T', ' ')}:00`;
};

type TemporalPickerProps = {
  value: any;
  dataType: 'date' | 'datetime';
  disabled: boolean;
  onInput: (value: string) => void;
};

const TemporalPicker = (props: TemporalPickerProps) => {
  let inputRef: HTMLInputElement | undefined;
  let pickerInstance: any;

  const getDateFormat = () => (props.dataType === 'date' ? 'Y-m-d' : 'Y-m-d H:i:S');
  const getAltFormat = () => (props.dataType === 'date' ? 'M j, Y' : 'M j, Y h:i K');
  const getNormalizedValue = () =>
    props.dataType === 'date' ? toDateInputValue(props.value) : toDateTimePickerValue(props.value);

  onMount(async () => {
    if (!inputRef) {
      return;
    }

    const normalizedValue = getNormalizedValue();
    if (normalizedValue) {
      inputRef.value = normalizedValue;
    }

    const flatpickrModule = await import('flatpickr');
    const flatpickr = flatpickrModule.default;

    pickerInstance = flatpickr(inputRef, {
      enableTime: props.dataType === 'datetime',
      enableSeconds: props.dataType === 'datetime',
      noCalendar: false,
      dateFormat: getDateFormat(),
      altInput: true,
      altFormat: getAltFormat(),
      allowInput: true,
      clickOpens: !props.disabled,
      defaultDate: normalizedValue || undefined,
      onChange: (_selectedDates: Date[], dateStr: string) => {
        props.onInput(dateStr);
      },
      onClose: (_selectedDates: Date[], dateStr: string) => {
        props.onInput(dateStr);
      },
    });
  });

  createEffect(() => {
    const normalizedValue = getNormalizedValue();

    if (pickerInstance) {
      pickerInstance.set('clickOpens', !props.disabled);
      pickerInstance.input.disabled = props.disabled;
      pickerInstance.altInput.disabled = props.disabled;
      pickerInstance.setDate(normalizedValue || '', false, getDateFormat());
      return;
    }

    if (inputRef) {
      inputRef.value = normalizedValue;
      inputRef.disabled = props.disabled;
    }
  });

  onCleanup(() => {
    if (pickerInstance) {
      pickerInstance.destroy();
    }
  });

  return <input ref={inputRef} type="text" />;
};

// Casting helper
const castValue = (value: string, type: DataType): any => {
  if (!value && value !== '0') return null;

  switch (type) {
    case 'number':
      const num = parseFloat(value);
      return isNaN(num) ? value : num;
    case 'boolean':
      return value === '1' || value === 'true';
    case 'date':
      return toDateInputValue(value);
    case 'datetime':
      if (/^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}$/.test(value)) {
        return `${value.replace('T', ' ')}:00`;
      }
      if (/^\d{4}-\d{2}-\d{2}\s\d{2}:\d{2}$/.test(value)) {
        return `${value}:00`;
      }
      return value;
    default:
      return value;
  }
};

const toComparableKey = (value: any): string => {
  if (value === null || value === undefined) {
    return '';
  }
  return String(value);
};

const getRowDisplayColumns = (row: Record<string, any>, explicitColumns?: string[]): string[] => {
  if (explicitColumns && explicitColumns.length > 0) {
    return explicitColumns;
  }
  return Object.keys(row);
};

const findReferencedRow = (
  foreignKey: ForeignKeyMetadata,
  value: any,
  relatedRows: RelatedRowsByTable,
): Record<string, any> | undefined => {
  const rows = relatedRows[foreignKey.referencesTable] || [];
  const target = toComparableKey(value);
  return rows.find((row) => toComparableKey(row[foreignKey.referencesColumn]) === target);
};

type EmbeddedRowPreviewProps = {
  table: string;
  row: Record<string, any>;
  relatedRows: RelatedRowsByTable;
  relatedColumns: RelatedColumnsByTable;
  depth: number;
};

const EmbeddedRowPreview = (props: EmbeddedRowPreviewProps) => {
  const columns = createMemo(() => getRowDisplayColumns(props.row, props.relatedColumns[props.table]));
  const foreignKeyMap = createMemo(() => {
    const map: Record<string, ForeignKeyMetadata> = {};
    getForeignKeys(props.table).forEach((foreignKey) => {
      map[foreignKey.column] = foreignKey;
    });
    return map;
  });

  return (
    <div class="fk-embed-container">
      <table class="fk-embed-table">
        <tbody>
          <tr class="fk-embed-header-row">
            <For each={columns()}>
              {(column) => <td>{formatColumnHeader(column)}</td>}
            </For>
          </tr>
          <tr>
            <For each={columns()}>
              {(column) => {
                const foreignKey = foreignKeyMap()[column];
                const value = props.row[column];

                if (!foreignKey || props.depth >= FK_MAX_RELATIONSHIP_DEPTH) {
                  return <td>{String(value)}</td>;
                }

                const referencedRow = findReferencedRow(foreignKey, value, props.relatedRows);

                if (!referencedRow) {
                  return <td>{String(value)}</td>;
                }

                return (
                  <td>
                    <EmbeddedRowPreview
                      table={foreignKey.referencesTable}
                      row={referencedRow}
                      relatedRows={props.relatedRows}
                      relatedColumns={props.relatedColumns}
                      depth={props.depth + 1}
                    />
                  </td>
                );
              }}
            </For>
          </tr>
        </tbody>
      </table>
    </div>
  );
};

type ForeignKeyPickerProps = {
  foreignKey: ForeignKeyMetadata;
  value: any;
  rows: Record<string, any>[];
  columns: string[];
  relatedRows: RelatedRowsByTable;
  relatedColumns: RelatedColumnsByTable;
  disabled: boolean;
  onConfirm: (value: any) => void;
};

const ForeignKeyPicker = (props: ForeignKeyPickerProps) => {
  const [open, setOpen] = createSignal(false);
  const [dropdownStyle, setDropdownStyle] = createSignal<Record<string, string>>({});
  let buttonRef: HTMLButtonElement | undefined;
  let dropdownRef: HTMLDivElement | undefined;

  const selectedRow = createMemo(() => {
    const valueKey = toComparableKey(props.value);
    if (!valueKey) {
      return undefined;
    }
    return props.rows.find((row) => toComparableKey(row[props.foreignKey.referencesColumn]) === valueKey);
  });

  const handleRowClick = (row: Record<string, any>) => {
    props.onConfirm(row[props.foreignKey.referencesColumn]);
    setOpen(false);
  };

  const updateDropdownPosition = () => {
    if (!buttonRef) {
      return;
    }

    const rect = buttonRef.getBoundingClientRect();
    const viewportPadding = 12;
    const dropdownGap = 4;
    const estimatedDropdownHeight = 260;

    const availableBelow = window.innerHeight - rect.bottom - viewportPadding;
    const availableAbove = rect.top - viewportPadding;
    const shouldOpenAbove = availableBelow < estimatedDropdownHeight && availableAbove > availableBelow;

    const left = Math.max(viewportPadding, rect.left);
    const maxWidth = Math.max(240, window.innerWidth - left - viewportPadding);
    const top = shouldOpenAbove ? rect.top - dropdownGap : rect.bottom + dropdownGap;

    setDropdownStyle({
      position: 'fixed',
      top: `${top}px`,
      left: `${left}px`,
      transform: shouldOpenAbove ? 'translateY(-100%)' : 'none',
      'min-width': `${rect.width}px`,
      'max-width': `${maxWidth}px`,
      'z-index': '5000',
    });
  };

  createEffect(() => {
    if (!open()) {
      return;
    }

    updateDropdownPosition();

    const handleWindowChange = () => updateDropdownPosition();
    const handleClickOutside = (event: MouseEvent) => {
      const target = event.target as Node | null;
      if (!target) {
        return;
      }

      const isInButton = !!buttonRef?.contains(target);
      const isInDropdown = !!dropdownRef?.contains(target);
      if (!isInButton && !isInDropdown) {
        setOpen(false);
      }
    };
    const handleEscape = (event: KeyboardEvent) => {
      if (event.key === 'Escape') {
        setOpen(false);
      }
    };

    window.addEventListener('resize', handleWindowChange);
    window.addEventListener('scroll', handleWindowChange, true);
    document.addEventListener('mousedown', handleClickOutside);
    document.addEventListener('keydown', handleEscape);

    onCleanup(() => {
      window.removeEventListener('resize', handleWindowChange);
      window.removeEventListener('scroll', handleWindowChange, true);
      document.removeEventListener('mousedown', handleClickOutside);
      document.removeEventListener('keydown', handleEscape);
    });
  });

  return (
    <div class="fk-picker">
      <button
        ref={buttonRef}
        type="button"
        class="fk-picker-button"
        onClick={() => setOpen((current) => !current)}
        disabled={props.disabled}
      >
        <span class="fk-picker-button-value">
          {selectedRow() ? (
            <EmbeddedRowPreview
              table={props.foreignKey.referencesTable}
              row={selectedRow()!}
              relatedRows={props.relatedRows}
              relatedColumns={props.relatedColumns}
              depth={0}
            />
          ) : (
            <span class="fk-picker-placeholder">Select {props.foreignKey.referencesTable}</span>
          )}
        </span>
        <span classList={{ 'fk-picker-arrow': true, 'fk-picker-arrow-open': open() }}>▾</span>
      </button>
      {open() && (
        <Portal>
          <div ref={dropdownRef} class="fk-picker-dropdown fk-picker-dropdown-portal" style={dropdownStyle()}>
            <div class="fk-picker-scroll">
              <table class="fk-picker-table">
                <thead>
                  <tr>
                    <For each={props.columns}>
                      {(column) => <th>{formatColumnHeader(column)}</th>}
                    </For>
                  </tr>
                </thead>
                <tbody>
                  <For each={props.rows}>
                    {(row) => {
                      const selectedValue = selectedRow()
                        ? toComparableKey(selectedRow()![props.foreignKey.referencesColumn])
                        : '';
                      const currentValue = toComparableKey(row[props.foreignKey.referencesColumn]);
                      return (
                        <tr
                          classList={{ 'fk-picker-selected': selectedValue === currentValue }}
                          onClick={() => handleRowClick(row)}
                        >
                          <For each={getRowDisplayColumns(row, props.columns)}>
                            {(column) => <td>{String(row[column])}</td>}
                          </For>
                        </tr>
                      );
                    }}
                  </For>
                </tbody>
              </table>
            </div>
          </div>
        </Portal>
      )}
    </div>
  );
};

type DataRowProps = {
  rowIndex: number;
  row: Record<string, any>;
  columns: string[];
  columnTypes: Record<string, DataType>;
  primaryKeys: string[];
  table: string;
  foreignKeyMap: Record<string, ForeignKeyMetadata>;
  relatedRows: RelatedRowsByTable;
  relatedColumns: RelatedColumnsByTable;
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
        const rawValue = editData()[col];
        const normalizedValue = rawValue === null || rawValue === undefined ? '' : String(rawValue);
        castData[col] = castValue(normalizedValue, props.columnTypes[col]);
      });
      await props.onUpdate(props.rowIndex, castData);
      props.setEditingId(null);
    } catch (err) {
      setError(String(err));
    } finally {
      setLoading(false);
    }
  };

  const renderDisplayValue = (column: string) => {
    const foreignKey = props.foreignKeyMap[column];
    const value = props.row[column];

    if (!foreignKey) {
      if (props.columnTypes[column] === 'date') {
        return formatDateForDisplay(value);
      }
      if (props.columnTypes[column] === 'datetime') {
        return formatDateTimeForDisplay(value);
      }
      return String(value);
    }

    const referencedRow = findReferencedRow(foreignKey, value, props.relatedRows);

    if (!referencedRow) {
      return String(value);
    }

    return (
      <EmbeddedRowPreview
        table={foreignKey.referencesTable}
        row={referencedRow}
        relatedRows={props.relatedRows}
        relatedColumns={props.relatedColumns}
        depth={0}
      />
    );
  };

  return (
    <>
      <tr>
        <For each={props.columns}>
          {(column) => (
            <td classList={{ 'fk-cell': !isEditing() && Boolean(props.foreignKeyMap[column]) }}>
              {isEditing() ? (
                props.foreignKeyMap[column] ? (
                  <ForeignKeyPicker
                    foreignKey={props.foreignKeyMap[column]}
                    value={editData()[column]}
                    rows={props.relatedRows[props.foreignKeyMap[column].referencesTable] || []}
                    columns={props.relatedColumns[props.foreignKeyMap[column].referencesTable] || []}
                    relatedRows={props.relatedRows}
                    relatedColumns={props.relatedColumns}
                    disabled={loading()}
                    onConfirm={(value) => setEditData((prev) => ({ ...prev, [column]: value }))}
                  />
                ) : (
                  props.columnTypes[column] === 'date' ? (
                    <TemporalPicker
                      value={editData()[column]}
                      dataType="date"
                      disabled={loading()}
                      onInput={(value) => handleInputChange(column, value)}
                    />
                  ) : props.columnTypes[column] === 'datetime' ? (
                    <TemporalPicker
                      value={editData()[column]}
                      dataType="datetime"
                      disabled={loading()}
                      onInput={(value) => handleInputChange(column, value)}
                    />
                  ) : (
                    <input
                      type="text"
                      value={editData()[column] ?? ''}
                      onInput={(e) => handleInputChange(column, e.currentTarget.value)}
                      disabled={loading()}
                    />
                  )
                )
              ) : (
                renderDisplayValue(column)
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
  columnTypes: Record<string, DataType>;
  foreignKeyMap: Record<string, ForeignKeyMetadata>;
  relatedRows: RelatedRowsByTable;
  relatedColumns: RelatedColumnsByTable;
  table: string;
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
        const rawValue = formData()[col];
        const normalizedValue = rawValue === null || rawValue === undefined ? '' : String(rawValue);

        // Skip auto-increment fields if empty (let DB generate)
        if (isAutoIncrement(props.table, col) && normalizedValue === '') {
          return;
        }

        // Convert empty nullable fields to NULL
        if (isNullable(props.table, col) && normalizedValue === '') {
          castData[col] = null;
          return;
        }

        // Default NOT NULL string fields to empty string
        if (normalizedValue === '' && props.columnTypes[col] === 'string') {
          castData[col] = '';
          return;
        }

        castData[col] = castValue(normalizedValue, props.columnTypes[col]);
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
              {props.foreignKeyMap[column] ? (
                <ForeignKeyPicker
                  foreignKey={props.foreignKeyMap[column]}
                  value={formData()[column]}
                  rows={props.relatedRows[props.foreignKeyMap[column].referencesTable] || []}
                  columns={props.relatedColumns[props.foreignKeyMap[column].referencesTable] || []}
                  relatedRows={props.relatedRows}
                  relatedColumns={props.relatedColumns}
                  disabled={loading()}
                  onConfirm={(value) => handleInputChange(column, String(value))}
                />
              ) : (
                props.columnTypes[column] === 'date' ? (
                  <TemporalPicker
                    value={formData()[column] || ''}
                    dataType="date"
                    disabled={loading()}
                    onInput={(value) => handleInputChange(column, value)}
                  />
                ) : props.columnTypes[column] === 'datetime' ? (
                  <TemporalPicker
                    value={formData()[column] || ''}
                    dataType="datetime"
                    disabled={loading()}
                    onInput={(value) => handleInputChange(column, value)}
                  />
                ) : (
                  <input
                    type="text"
                    placeholder={
                      isAutoIncrement(props.table, column)
                        ? `${formatColumnHeader(column)} (auto)`
                        : isNullable(props.table, column)
                          ? `${formatColumnHeader(column)} (optional)`
                          : formatColumnHeader(column)
                    }
                    value={formData()[column] || ''}
                    onInput={(e) => handleInputChange(column, e.currentTarget.value)}
                    disabled={loading()}
                  />
                )
              )}
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
  const [columnTypes, setColumnTypes] = createSignal<Record<string, DataType>>({});
  const [relatedRows, setRelatedRows] = createSignal<RelatedRowsByTable>({});
  const [relatedColumns, setRelatedColumns] = createSignal<RelatedColumnsByTable>({});
  const [refetchCount, setRefetchCount] = createSignal<number>(0);
  const primaryKeys = getPrimaryKeys(props.table);
  const foreignKeyMap = createMemo(() => {
    const map: Record<string, ForeignKeyMetadata> = {};
    getForeignKeys(props.table).forEach((foreignKey) => {
      map[foreignKey.column] = foreignKey;
    });
    return map;
  });
  const relatedTables = createMemo(() => {
    return Array.from(new Set(getForeignKeys(props.table).map((foreignKey) => foreignKey.referencesTable)));
  });

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

  const [relatedData] = createResource(
    () => ({ tables: relatedTables(), refetch: refetchCount() }),
    async ({ tables }) => {
      const entries = await Promise.all(
        tables.map(async (table) => {
          const response = await fetch(getApiUrl(table));
          if (!response.ok) {
            throw new Error(`Failed to fetch table data: ${response.statusText}`);
          }
          const rows = await response.json();
          return [table, rows as Record<string, any>[]] as const;
        })
      );

      return Object.fromEntries(entries) as RelatedRowsByTable;
    }
  );

  createEffect(() => {
    if (data()) {
      setTableData([...data()]);
      if (data().length > 0) {
        const cols = Object.keys(data()[0]);
        setColumns(cols);

        // Detect types from first row
        const types: Record<string, DataType> = {};
        cols.forEach(col => {
          types[col] = detectType(data()[0][col], col, getColumnDataType(props.table, col));
        });
        setColumnTypes(types);
      }
    }
  });

  createEffect(() => {
    if (relatedData()) {
      setRelatedRows(relatedData()!);

      const columnsByTable: RelatedColumnsByTable = {};
      Object.entries(relatedData()!).forEach(([table, rows]) => {
        columnsByTable[table] = rows.length > 0 ? Object.keys(rows[0]) : [];
      });
      setRelatedColumns(columnsByTable);
    }
  });

  onMount(() => {
    const eventSource = new EventSource('/api/stream');

    const handleDbUpdate = (event: MessageEvent) => {
      try {
        const payload = JSON.parse(event.data) as { table?: string };
        if (payload.table === props.table || relatedTables().includes(payload.table || '') || payload.table === '*') {
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
                    foreignKeyMap={foreignKeyMap()}
                    relatedRows={relatedRows()}
                    relatedColumns={relatedColumns()}
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
                foreignKeyMap={foreignKeyMap()}
                relatedRows={relatedRows()}
                relatedColumns={relatedColumns()}
                table={props.table}
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