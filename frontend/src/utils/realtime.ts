/*
 * Authored and maintained by GitHub Copilot (GPT-5.3-Codex)
 * Date: March 2, 2026
 */

type DbUpdateEvent = {
  table: string;
  action: 'insert' | 'update' | 'delete' | 'reset';
  timestamp: number;
};

type RealtimeListener = (event: DbUpdateEvent) => void;

const listeners = new Set<RealtimeListener>();

export const subscribeToDbUpdates = (listener: RealtimeListener): (() => void) => {
  listeners.add(listener);
  return () => {
    listeners.delete(listener);
  };
};

export const broadcastDbUpdate = (event: Omit<DbUpdateEvent, 'timestamp'>): void => {
  const payload: DbUpdateEvent = {
    ...event,
    timestamp: Date.now(),
  };

  listeners.forEach((listener) => {
    try {
      listener(payload);
    } catch (error) {
      console.error('Realtime listener error:', error);
    }
  });
};
