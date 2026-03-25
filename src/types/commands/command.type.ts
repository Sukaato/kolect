export type Command<Params, Result, Error = string> = {
  params: CamelCase<Params>
  result: Result
  error?: Error
}

type CamelCase<T> = {
  [K in keyof T as SnakeToCamel<K & string>]: T[K] extends Record<string, unknown>
    ? CamelCase<T[K]>
    : T[K]
}

type SnakeToCamel<S extends string> = S extends `${infer Head}_${infer Tail}`
  ? `${Head}${Capitalize<SnakeToCamel<Tail>>}`
  : S

export type InferCommand<T> = {
  [K in keyof T]: T[K] extends Command<infer P, infer R, infer E>
    ? {
        params: CamelCase<P>
        result: R
        error: E
      }
    : never
}
