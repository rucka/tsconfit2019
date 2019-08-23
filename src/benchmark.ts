import { BenchmarkConfiguration } from './configuration'
import { Processor } from './api'
import { runner } from './misc'

function* orderIdIterator(
  ids: { ok: string[]; ko: string[] },
  failureRate: number,
  iterations: number
) {
  let okIndex = 0
  let koIndex = 0
  const failAtModule = failureRate * 100
  for (let i = 0; i < iterations; i++) {
    if (failAtModule > 0 && i > 0 && i % failAtModule === 0) {
      const current = koIndex++ % ids.ko.length
      yield ids.ko[current]
    } else {
      const current = okIndex++ % ids.ok.length
      yield ids.ok[current]
    }
  }
}

const benchmark = async (configuration: BenchmarkConfiguration) => {
  const processor: Processor<void> = (orderId: string) =>
    configuration
      .processor(orderId)
      .then(() => {})
      .catch(() => {})

  if (configuration.warmup > 0) {
    const warmUpIterator = orderIdIterator(
      configuration.ids,
      configuration.failureRate,
      configuration.warmup
    )
    await runner(processor)(warmUpIterator)
  }
  const benchmarkIterator = orderIdIterator(
    configuration.ids,
    configuration.failureRate,
    configuration.epoch
  )
  const start = new Date().getTime()
  await runner(processor)(benchmarkIterator)
  const end = new Date().getTime()
  return end - start
}
export default benchmark
