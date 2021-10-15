export function getBackendUrl(): string {
  return process.env.REACT_APP_BACKEND_URL ?? "";
}
