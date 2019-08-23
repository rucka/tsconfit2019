import { get as getConfiguration } from './configuration'
import benchmark from './benchmark'
;(async () => {
  const configuration = await getConfiguration()
  const timeInMs = await benchmark(configuration)
  console.log(
    `${configuration.name}\t\t${configuration.failureRate}\t\t${configuration.warmup}\t\t${configuration.epoch}\t\t${timeInMs} ms`
  )
  process.exit(0)
})()
