import { get as getConfiguration } from './configuration'
import benchmark from './benchmark'
;(async () => {
  const configuration = await getConfiguration()
  const timeInMs = await benchmark(configuration)
  console.log(`the test ${configuration.name} ends in ${timeInMs}ms`)
})()
