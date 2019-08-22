import { Processor } from './api'

export const runner = <T>(processor: Processor<T>) => async (ids: IterableIterator<string> | Iterable<string>) => {
  const result: T[] = []
  for (let id of ids) {
    const r = await processor(id)
    result.push(r)
  }
  return result
}
