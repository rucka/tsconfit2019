import processorVanilla from './processOrderVanilla'
import porcessorFp from './porcessOrderFp'
import { categorizedOrderIds } from './data'
import { Processor } from './api'
import { readFileSync } from 'fs'

const config = JSON.parse(readFileSync(__dirname + '/params.json', 'utf8')) as {
  warmup: number
  epoch: number
  failureRate: number
}

const processName = process.argv[2]

const getProcessor = (processName: string) => {
  if (processName === 'vanilla') return processorVanilla
  if (processName === 'fp') return porcessorFp
  throw new Error('Process not recognized')
}
const processor = getProcessor(processName)

export type BenchmarkIds = { ok: string[]; ko: string[] }
export type BenchmarkConfiguration = {
  name: string
  processor: Processor
  warmup: number
  failureRate: number
  epoch: number
}

export async function get(): Promise<BenchmarkConfiguration> {
  return {
    name: processName,
    processor: processor,
    warmup: config.warmup,
    epoch: config.epoch,
    failureRate: config.failureRate
  }
}

export type RunnerResult = {
  ok_counter: number
  ko_counter: number
  total: number
}

export async function runner(
  processor: Processor,
  iterations: number,
  failure_rate: number,
  ids: BenchmarkIds
): Promise<RunnerResult> {
  let ok_counter = 0
  let ko_counter = 0
  let total = 0.0

  while (ok_counter + ko_counter < iterations) {
    let id = ''
    if (ok_counter > 0 && ko_counter / ok_counter < failure_rate) {
      id = ids.ko[ko_counter % ids.ko.length]
      ko_counter += 1
    } else {
      id = ids.ok[ok_counter % ids.ok.length]
      ok_counter += 1
    }
    total += (await processor(id)).totalAmount
  }
  return {
    ok_counter,
    ko_counter,
    total
  }
}

export async function benchmark(
  config: BenchmarkConfiguration
): Promise<[number, RunnerResult]> {
  await runner(
    config.processor,
    config.warmup,
    config.failureRate,
    categorizedOrderIds
  )
  const start = new Date().getTime()
  let result = await runner(
    config.processor,
    config.epoch,
    config.failureRate,
    categorizedOrderIds
  )
  const end = new Date().getTime()
  return [end - start, result]
}
