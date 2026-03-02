/*
 * Authored and maintained by GitHub Copilot (GPT-5.3-Codex)
 * Date: March 2, 2026
 */

import { subscribeToDbUpdates } from '~/utils/realtime';

const encoder = new TextEncoder();

const encodeEvent = (event: string, payload: unknown): Uint8Array => {
  return encoder.encode(`event: ${event}\ndata: ${JSON.stringify(payload)}\n\n`);
};

export async function GET() {
  let cleanup: (() => void) | undefined;
  let keepAliveTimer: ReturnType<typeof setInterval> | undefined;

  const stream = new ReadableStream<Uint8Array>({
    start(controller) {
      controller.enqueue(encodeEvent('ready', { ok: true, timestamp: Date.now() }));

      cleanup = subscribeToDbUpdates((event) => {
        controller.enqueue(encodeEvent('db-update', event));
      });

      keepAliveTimer = setInterval(() => {
        controller.enqueue(encoder.encode(`: ping ${Date.now()}\n\n`));
      }, 25000);
    },
    cancel() {
      if (keepAliveTimer) {
        clearInterval(keepAliveTimer);
      }
      if (cleanup) {
        cleanup();
      }
    },
  });

  return new Response(stream, {
    headers: {
      'Content-Type': 'text/event-stream',
      'Cache-Control': 'no-cache, no-transform',
      Connection: 'keep-alive',
    },
  });
}
