import { useInvoke } from './use-invoke'

export interface LogConfig {
  level: string
}

export interface DatasetConfig {
  url: string
}

export interface Config {
  log: LogConfig
  dataset: DatasetConfig
}

export function useConfig() {
  const { result, error, loading, invoke } = useInvoke<Config>('get_config')

  return {
    config: result,
    loading,
    error,
    load: invoke,
  }
}
