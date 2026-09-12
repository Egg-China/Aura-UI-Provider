/// Extracts non-empty Microsoft profile names from a launcher snapshot.
///
/// Malformed entries are ignored instead of receiving mock display defaults.
export function microsoftAccountSuggestions(raw: unknown): string[] {
  if (!Array.isArray(raw)) return [];

  const seen = new Set<string>();
  const suggestions: string[] = [];
  for (const candidate of raw) {
    if (typeof candidate !== 'object' || candidate === null) continue;
    const entry = candidate as Record<string, unknown>;
    if (entry.type !== 'microsoft' || typeof entry.username !== 'string') continue;

    const username = entry.username.trim();
    const key = username.toLowerCase();
    if (username.length === 0 || seen.has(key)) continue;
    seen.add(key);
    suggestions.push(username);
  }
  return suggestions;
}

/// Removes suggestions that already match an active backend account profile name.
export function availableMicrosoftSuggestions(
  suggestions: string[],
  accounts: { username: string }[],
): string[] {
  const existing = new Set(
    accounts.flatMap((account) => (
      typeof account.username === 'string' ? [account.username.trim().toLowerCase()] : []
    )),
  );
  return suggestions.filter((username) => !existing.has(username.trim().toLowerCase()));
}
