declare const __brand: unique symbol
export type Brand<Type extends any, Name extends string> = Type & { [__brand]: Name }
