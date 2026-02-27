import { useInvoke } from './use-invoke'

export interface DatasetConfig {
  url: string
}

export interface Config {
  dataset: DatasetConfig
}

export function useConfig() {
  const { result, error, loading, invoke } = useInvoke<Config>('config_get')

  return {
    config: result,
    loading,
    error,
    load: invoke,
  }
}
