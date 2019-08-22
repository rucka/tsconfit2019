import { orders } from './data'

import processOrderVanilla from './processOrderVanilla'
import porcessOrderFp from './porcessOrderFp'
import { ProcessOrder } from './api'
import { runner } from './misc'

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
const orderKeys = Object.keys(orders)
const splitIds = async (orderKeys: string[]) =>
  runner(processor)(orderKeys).then(xs => {
    const ok: string[] = []
    const ko: string[] = []
    xs.map(x => {
      if (x.success) {
        ok.push(x.orderId)
      } else {
        ko.push(x.orderId)
      }
    })
    return { ok, ko }
  })

export async function get(): Promise<BenchmarkConfiguration> {
  const ids = await splitIds(orderKeys)
  return {
    name: processName,
    processor: processOrder,
    warmup: 0,
    epoch: 200000,
    failureRate: 0.01,
    ids
  }
}
