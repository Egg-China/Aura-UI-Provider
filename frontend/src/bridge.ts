/// Bridge helpers for the optional Tauri/aura.ui.v1 host.

/// One launcher-registered plugin contribution rendered by the Modern UI.
export interface PluginContribution {
    id: string;
    pluginId?: string;
    kind: 'sidebar' | 'button';
    label: string;
    icon?: string;
    action?: string;
}

/// Shape of the `core.snapshot.get` payload produced by the Java launcher.
export interface LauncherSnapshot {
    instances?: unknown[];
    accounts?: unknown[];
    settings?: Record<string, unknown>;
    pluginContributions?: PluginContribution[];
}

/// Sends one `core.*` request through the native transport, returning parsed JSON.
export async function bridgeRequest<T = unknown>(method: string, params: unknown = null): Promise<T> {
    const { invoke } = await import('@tauri-apps/api/core');
    const raw = await invoke<string>('frontend_request', {
        method,
        paramsJson: JSON.stringify(params),
    });
    return JSON.parse(raw) as T;
}

/// Parses a launcher snapshot defensively; `null` keeps the local mock state.
export function parseSnapshot(raw: string): LauncherSnapshot | null {
    try {
        const parsed = JSON.parse(raw) as unknown;
        if (typeof parsed === 'object' && parsed !== null) {
            return parsed as LauncherSnapshot;
        }
        return null;
    } catch {
        return null;
    }
}
