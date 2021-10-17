import { getBackendUrl } from "./env";
import { Item } from "./types/item";
import { isNil } from "./utils";

export type FetchError = {
  status: number;
  body: unknown;
};

export function isFetchError(error: unknown): error is FetchError {
  return (
    !isNil(error) &&
    typeof error === "object" &&
    typeof (error as Record<string, unknown>).status === "number"
  );
}

function fetchJson<T>(url: string, options?: RequestInit): Promise<T> {
  const mergedOptions = {
    headers: {
      Accept: "application/json",
      "Content-Type": "application/json",
    },
    ...options,
  };

  return fetch(url, mergedOptions)
    .then((response) =>
      response.json().then((body: unknown) => {
        if (Math.floor(response.status / 100) !== 2) {
          const error: FetchError = {
            status: response.status,
            body,
          };

          return Promise.reject(error);
        }

        return body as T;
      })
    )
    .catch((e: unknown) => {
      if (isFetchError(e)) {
        return Promise.reject(e);
      }

      const message = e instanceof Error ? e.message : e;

      return Promise.reject({
        status: 0,
        body: message,
      });
    });
}

export function getItems(): Promise<Item[]> {
  return fetchJson<Item[]>(`${getBackendUrl()}/item`);
}

export function updateStockLoss(
  itemId: number,
  quantity: number,
  date?: Date
): Promise<number> {
  return fetchJson<number>(`${getBackendUrl()}/item/${itemId}/loss`, {
    method: "PUT",
    body: JSON.stringify({
      quantity: quantity,
      date: isNil(date)
        ? undefined
        : {
            day: date.getDate(),
            month: date.getMonth() + 1,
            year: date.getFullYear(),
          },
    }),
  });
}

export function updateStockResupply(
  itemId: number,
  quantity: number,
  date?: Date
): Promise<number> {
  return fetchJson<number>(`${getBackendUrl()}/item/${itemId}/resupply`, {
    headers: {
      Accept: "application/json",
      "Content-Type": "application/json",
    },
    method: "PUT",
    body: JSON.stringify({
      quantity: quantity,
      date: isNil(date)
        ? undefined
        : {
            day: date.getDate(),
            month: date.getMonth() + 1,
            year: date.getFullYear(),
          },
    }),
  });
}
