export type stringKey<T> = { [key: string]: T }

export interface Stringable {
    toString(): string;
}
