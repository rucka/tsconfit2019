import { orders } from './data'

import processOrderVanilla from './processOrderVanilla'
import porcessOrderFp from './porcessOrderFp'
import { ProcessOrder } from './api'
import { readFileSync } from 'fs'

const config = JSON.parse(readFileSync(__dirname + '/params.json', 'utf8')) as {
  warmup: number
  epoch: number
  failureRate: number
}

const processName = process.argv[2]

const getProcessOrder = (processName: string) => {
  if (processName === 'vanilla') return processOrderVanilla
  if (processName === 'fp') return porcessOrderFp
  throw new Error('Process not recognized')
}
const processOrder = getProcessOrder(processName)

const splitInErrorProcess = (f: ProcessOrder) => (orderId: string) =>
  f(orderId)
    .then(() => ({ success: true, orderId }))
    .catch(() => ({ success: false, orderId }))
const processor = splitInErrorProcess(processOrder)

export type BenchmarkConfiguration = {
  name: string
  processor: ProcessOrder
  warmup: number
  failureRate: number
  epoch: number
  ids: { ok: string[]; ko: string[] }
}

export async function get(): Promise<BenchmarkConfiguration> {
  const ok: string[] = []
  const ko: string[] = []

  for (let id of Object.keys(orders)) {
    const r = await processor(id)
    if (r.success) {
      ok.push(id)
    } else {
      ko.push(id)
    }
  }
  const ids = { ok, ko }

  return {
    name: processName,
    processor: processOrder,
    warmup: config.warmup,
    epoch: config.epoch,
    failureRate: config.failureRate,
    ids
  }
}
