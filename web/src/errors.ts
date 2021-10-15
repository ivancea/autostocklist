import { isFetchError } from "./api";

export function stringifyError(error: unknown): string {
  if (isFetchError(error)) {
    return JSON.stringify(error.body);
  }

  console.warn("Unknown error: " + JSON.stringify(error));
  return JSON.stringify(error);
}
