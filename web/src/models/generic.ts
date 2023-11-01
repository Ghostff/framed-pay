export type stringKey<T> = { [key: string]: T }

export interface Stringable {
    toString(): string;
}

export interface BasicModalInterface {
    open: () => void,
    close: () => void,
    toggle: (flag: boolean) => void,
}