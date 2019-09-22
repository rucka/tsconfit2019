import processorVanilla from './processOrderVanilla'
import processorVanillaSync from './processOrderVanillaSync'
import processorFp from './processOrderFp'
import processorFpChecked from './processOrderFpChecked'
import { categorizedOrderIds } from './data'
import { SyncProcessor, AsyncProcessor } from './api'
import { readFileSync } from 'fs'

const config = JSON.parse(readFileSync(__dirname + '/params.json', 'utf8')) as {
  warmup: number
  epoch: number
  failureRate: number
}

function getAsyncProcessor(processorName: string): AsyncProcessor | null {
  if (processorName === 'vanilla') return processorVanilla
  if (processorName === 'fp') return processorFp
  if (processorName === 'checked') return processorFpChecked
  return null
}
function getSyncProcessor(processorName: string): SyncProcessor | null {
  if (processorName === 'syncv') return processorVanillaSync
  //if (processorName === 'syncfp') return porcessorFp
  return null
}

export type BenchmarkIds = { ok: string[]; ko: string[] }

export type SyncBenchmarkConfiguration = {
  isSync: true
  name: string
  processor: SyncProcessor
  warmup: number
  failureRate: number
  epoch: number
}
export type AsyncBenchmarkConfiguration = {
  isSync: false
  name: string
  processor: AsyncProcessor
  warmup: number
  failureRate: number
  epoch: number
}
export type BenchmarkConfiguration =
  | SyncBenchmarkConfiguration
  | AsyncBenchmarkConfiguration

export function get(): BenchmarkConfiguration {
  const processorName = process.argv[2]

  let syncProcessor = getSyncProcessor(processorName)
  if (syncProcessor !== null) {
    return {
      isSync: true,
      name: processorName,
      processor: syncProcessor,
      warmup: config.warmup,
      epoch: config.epoch,
      failureRate: config.failureRate
    }
  }

  let asyncProcessor = getAsyncProcessor(processorName)
  if (asyncProcessor !== null) {
    return {
      isSync: false,
      name: processorName,
      processor: asyncProcessor,
      warmup: config.warmup,
      epoch: config.epoch,
      failureRate: config.failureRate
    }
  }

  throw new Error('Processor not recognized')
}

export type RunnerResult = {
  ok_counter: number
  ko_counter: number
  total: number
}

export function syncRunner(
  processor: SyncProcessor,
  iterations: number,
  failure_rate: number,
  ids: BenchmarkIds
): RunnerResult {
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
    total += processor(id).totalAmount
  }
  return {
    ok_counter,
    ko_counter,
    total
  }
}

export async function asyncRunner(
  processor: AsyncProcessor,
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
  if (config.isSync) {
    syncRunner(
      config.processor,
      config.warmup,
      config.failureRate,
      categorizedOrderIds
    )
    const start = new Date().getTime()
    let result = syncRunner(
      config.processor,
      config.epoch,
      config.failureRate,
      categorizedOrderIds
    )
    const end = new Date().getTime()
    return [end - start, result]
  } else {
    await asyncRunner(
      config.processor,
      config.warmup,
      config.failureRate,
      categorizedOrderIds
    )
    const start = new Date().getTime()
    let result = await asyncRunner(
      config.processor,
      config.epoch,
      config.failureRate,
      categorizedOrderIds
    )
    const end = new Date().getTime()
    return [end - start, result]
  }
}
