import { get as getConfiguration, benchmark } from './configuration'

var memwatch = require('node-memwatch')
;(async () => {
  const heapDiffStart = new memwatch.HeapDiff()
  const configuration = await getConfiguration()
  const [timeInMs, result] = await benchmark(configuration)
  const heapDiff = heapDiffStart.end()
  const iterInUsec = (1000 * timeInMs) / configuration.epoch
  const heapUsage = (heapDiff.after.size_bytes - heapDiff.before.size_bytes)
    .toString()
    .padStart(8, ' ')
  console.log(
    `${configuration.name}\ttime ms ${timeInMs}\t iter us ${iterInUsec}\theap ${heapUsage}\twarmup \t${configuration.warmup}\titer ${configuration.epoch}\t(ok ${result.ok_counter} ko ${result.ko_counter})\ttotal ${result.total}`
  )
  process.exit(0)
})()
