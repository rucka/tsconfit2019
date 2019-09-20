import { get as getConfiguration, benchmark } from './configuration'
;(async () => {
  const configuration = await getConfiguration()
  const [timeInMs, result] = await benchmark(configuration)
  let iterInUsec = (1000 * timeInMs) / configuration.epoch
  console.log(
    `${configuration.name}\ttime ms ${timeInMs}\t iter us ${iterInUsec}\twarmup \t${configuration.warmup}\titer ${configuration.epoch}\t(ok ${result.ok_counter} ko ${result.ko_counter})\ttotal ${result.total}`
  )
  process.exit(0)
})()
