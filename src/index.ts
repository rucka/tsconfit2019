import { orders } from './data'
import { ProcessOrder } from './api'
import processOrderVanilla from './processOrderVanilla'
import porcessOrderFp from './porcessOrderFp'

const process = (f: ProcessOrder) => (orderId: string) =>
  f(orderId)
    .then(result =>
      console.log(`order ${orderId} total amount: ${JSON.stringify(result)}`)
    )
    .catch((e: Error) => console.log(`Error order ${orderId} ${e.message}`))

const process1 = process(processOrderVanilla)
const process2 = process(porcessOrderFp)

const orderKeys = Object.keys(orders)

Promise.all(orderKeys.map(process1))
  .then(() => {
    console.log('completed vanilla processing...\n*******************')
  })
  .then(() =>
    Promise.all(orderKeys.map(process2)).then(() => {
      console.log('completed fp processing...')
    })
  )
