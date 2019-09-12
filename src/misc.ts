import { Processor } from './api'

export const runner = <T>(processor: Processor<T>) => async (
  ids: IterableIterator<string> | Iterable<string>
) => {
  for (let id of ids) {
    await processor(id)
  }
}
