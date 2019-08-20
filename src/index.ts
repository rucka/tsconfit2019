import { orders } from './data'
import { EvaluateOrder } from './api'
import processOrderVanilla from './processOrderVanilla'

const process = (f: EvaluateOrder) => (orderId: string) =>
  f(orderId)
    .then(result =>
      console.log(`order ${orderId} total amount: ${JSON.stringify(result)}`)
    )
    .catch((e: Error) => console.log(`Error order ${orderId} ${e.message}`))

const processOrder = process(processOrderVanilla)
Promise.all(Object.keys(orders).map(processOrder)).then(() => {
  console.log('completed...')
})
