import { Processor } from './api'

export const runner = (processor: Processor) => async (
  ids: IterableIterator<string> | Iterable<string>
) => {
  for (let id of ids) {
    await processor(id)
  }
}
