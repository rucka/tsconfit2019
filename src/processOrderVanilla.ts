import { orders, books } from './data'
import { validateOrder, Order, EvaluateOrder as ProcessOrder } from './api'

const orderService = (orderId: string) =>
  orders[orderId]
    ? Promise.resolve(orders[orderId])
    : Promise.reject(new Error(`Order not found: ${orderId}`))
const validationService = (order: Order) => {
  const r = validateOrder(order)
  if (r.valid) {
    return Promise.resolve(order)
  } else {
    return Promise.reject(new Error(`${r.error}`))
  }
}
const calculateAmountService = (order: Order) => {
  let total = 0
  for (let i = 0; i < order.items.length; i++) {
    const item = order.items[i]
    total += item.quantity * books[item.bookId].price
  }
  return Promise.resolve(total)
}

export const processOrderVanilla: ProcessOrder = (orderId: string) =>
  orderService(orderId)
    .then(validationService)
    .then(calculateAmountService)
