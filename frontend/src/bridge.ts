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

/// AuraCore native-engine status returned by `core.auracore.status`.
export interface AuraCoreStatus {
    engine: string;
    dataDirectory: string;
    libraryPath: string;
    libraryAvailable: boolean;
    backendRunning: boolean;
    migrationAllowList: string[];
}

/// One backend instance entry from `core.auracore.instance.list`.
export interface AuraCoreInstance {
    id: string;
    name: string;
    dir?: string;
    icon?: string;
    group?: string;
    lastLaunch?: number;
    gameVersion?: string;
    loader?: string;
    loaderVersion?: string;
    modCount?: number;
}

/// One backend account entry from `core.auracore.accounts.list`.
export interface AuraCoreAccount {
    profileName: string;
    type: string;
    internalId: string;
    hasProfile: boolean;
    isDefault?: boolean;
}

/// Task snapshot from `core.auracore.task.status`.
export interface AuraCoreTaskStatus {
    id: string;
    type: string;
    state: 'running' | 'succeeded' | 'failed' | 'aborted';
    progress: number;
    total: number;
    status: string;
    succeeded?: boolean;
    error?: string;
}

/// Log tail reply from `core.auracore.instance.logs`.
export interface AuraCoreLogReply {
    id: string;
    running: boolean;
    total?: number;
    logs: { level: string; line: string }[];
}

/// Device-code login info from `core.auracore.auth.msa.info`.
export interface AuraCoreMsaInfo {
    id: string;
    codeIssued: boolean;
    verificationUrl?: string;
    userCode?: string;
    expiresIn?: number;
}

/// Rejects when a successful bridge reply carries the backend `{ error }` envelope.
function rejectBackendError<T>(reply: T): T {
    if (typeof reply === 'object' && reply !== null) {
        const candidate = reply as Record<string, unknown>;
        if (typeof candidate.error === 'string') {
            throw new Error(candidate.error);
        }
    }
    return reply;
}

/// Reads the AuraCore engine, library, and migration status.
export async function auraCoreStatus(): Promise<AuraCoreStatus> {
    return rejectBackendError(await bridgeRequest<AuraCoreStatus>('core.auracore.status'));
}

/// Lists the instances owned by the AuraCore backend data directory.
export async function auraCoreListInstances(): Promise<AuraCoreInstance[]> {
    return rejectBackendError(await bridgeRequest<AuraCoreInstance[]>('core.auracore.instance.list'));
}

/// Lists the accounts stored inside the AuraCore backend.
export async function auraCoreListAccounts(): Promise<AuraCoreAccount[]> {
    return rejectBackendError(await bridgeRequest<AuraCoreAccount[]>('core.auracore.accounts.list'));
}

/// Creates a vanilla instance and returns the tracked backend task id.
export async function auraCoreCreateInstance(name: string, version: string, group?: string): Promise<string> {
    return rejectBackendError(
        await bridgeRequest<string>('core.auracore.instance.create', { name, version, group: group ?? null }),
    );
}

/// One selectable instance-tree entry from `core.instance.export.files.list`.
export interface ExportFileEntry {
    name: string;
    path: string;
    directory: boolean;
    suggested: boolean;
}

/// One bounded selection-tree level from `core.instance.export.files.list`.
export interface ExportFileListing {
    path: string;
    entries?: ExportFileEntry[];
    truncated?: boolean;
}

/// Lists one export selection-tree level for a launcher-side instance.
export async function listInstanceExportFiles(id: string, path = ''): Promise<ExportFileListing> {
    return rejectBackendError(
        await bridgeRequest<ExportFileListing>('core.instance.export.files.list', { id, path }),
    );
}

/// Exports one launcher-side instance as a MultiMC modpack archive.
///
/// A provided whitelist must be a non-empty array of exact selection paths; an undefined
/// whitelist keeps the launcher's full-export semantics.
export async function exportInstanceAsMultiMc(
    id: string,
    output: string,
    name?: string,
    whitelist?: string[],
): Promise<void> {
    if (whitelist !== undefined && whitelist.length === 0) {
        throw new Error('Export selection is empty');
    }
    const reply = rejectBackendError(
        await bridgeRequest<{ exported?: boolean }>('core.instance.export.multimc', {
            id,
            output,
            name: name ?? null,
            whitelist: whitelist ?? null,
        }),
    );
    if (reply.exported !== true) {
        throw new Error('MultiMC export failed');
    }
}

/// Starts a MultiMC-archive import and returns the tracked task id.
export async function auraCoreImportInstance(source: string, name: string, group?: string): Promise<string> {
    const reply = rejectBackendError(
        await bridgeRequest<{ taskId?: string }>('core.auracore.instance.import', {
            source,
            name,
            group: group ?? null,
        }),
    );
    if (typeof reply.taskId !== 'string' || reply.taskId.length === 0) {
        throw new Error('AuraCore did not return an import task id');
    }
    return reply.taskId;
}

/// Renames one AuraCore instance.
export async function auraCoreRenameInstance(id: string, name: string): Promise<void> {
    rejectBackendError(await bridgeRequest('core.auracore.instance.rename', { id, name }));
}

/// Deletes one AuraCore instance directory.
export async function auraCoreDeleteInstance(id: string): Promise<void> {
    rejectBackendError(await bridgeRequest('core.auracore.instance.delete', { id }));
}

/// Moves one AuraCore instance into a group; an empty group clears it.
export async function auraCoreSetInstanceGroup(id: string, group: string): Promise<void> {
    rejectBackendError(await bridgeRequest('core.auracore.instance.group', { id, group }));
}

/// Sets the icon key of one AuraCore instance.
export async function auraCoreSetInstanceIcon(id: string, icon: string): Promise<void> {
    rejectBackendError(await bridgeRequest('core.auracore.instance.icon', { id, icon }));
}

/// Tails live output of one AuraCore instance.
export async function auraCoreInstanceLogs(id: string, maxLines = 300): Promise<AuraCoreLogReply> {
    return rejectBackendError(await bridgeRequest<AuraCoreLogReply>('core.auracore.instance.logs', { id, maxLines }));
}

/// Terminates one running AuraCore game process.
export async function auraCoreStopInstance(id: string): Promise<void> {
    rejectBackendError(await bridgeRequest('core.auracore.instance.stop', { id }));
}

/// Reads one tracked AuraCore task.
export async function auraCoreTaskStatus(taskId: string): Promise<AuraCoreTaskStatus> {
    return rejectBackendError(await bridgeRequest<AuraCoreTaskStatus>('core.auracore.task.status', { taskId }));
}

/// Adds an offline account to the AuraCore backend.
export async function auraCoreAddOfflineAccount(username: string): Promise<void> {
    rejectBackendError(await bridgeRequest('core.auracore.accounts.add-offline', { username }));
}

/// Removes one AuraCore account by profile name.
export async function auraCoreRemoveAccount(profile: string): Promise<void> {
    rejectBackendError(await bridgeRequest('core.auracore.accounts.remove', { profile }));
}

/// Selects the AuraCore account used by future launches.
export async function auraCoreSetDefaultAccount(profile: string): Promise<void> {
    rejectBackendError(await bridgeRequest('core.auracore.accounts.set-default', { profile }));
}

/// Starts a Microsoft device-code login and returns its task id.
export async function auraCoreBeginMsaLogin(): Promise<string> {
    const reply = rejectBackendError(
        await bridgeRequest<{ started: boolean; taskId?: string }>('core.auracore.auth.msa.begin'),
    );
    if (!reply.started || typeof reply.taskId !== 'string') {
        throw new Error('AuraCore did not start the Microsoft login flow');
    }
    return reply.taskId;
}

/// Reads device-code login info for one login task.
export async function auraCoreMsaInfo(taskId: string): Promise<AuraCoreMsaInfo> {
    return rejectBackendError(await bridgeRequest<AuraCoreMsaInfo>('core.auracore.auth.msa.info', { taskId }));
}

/// Copies the allowlisted launcher settings into AuraCore.
export async function auraCoreMigrate(): Promise<Record<string, string>> {
    return rejectBackendError(await bridgeRequest<Record<string, string>>('core.auracore.migrate'));
}

/// Launches one instance, returning the AuraCore task id when the native engine handles it.
export async function launchInstance(id: string): Promise<string | null> {
    const reply = rejectBackendError(await bridgeRequest<unknown>('core.instance.launch', { id }));
    return typeof reply === 'string' && reply.length > 0 ? reply : null;
}
