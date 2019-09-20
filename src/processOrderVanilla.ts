import { orders, books } from './data'
import { validateOrder, Order, AsyncProcessor, PlacedOrderResult } from './api'

const bookService = (bookId: string) =>
  books[bookId]
    ? Promise.resolve(books[bookId])
    : Promise.reject(new Error(`Book not found: ${bookId}`))

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

const calculateAmountService = async (order: Order) => {
  let total = 0
  for (let i = 0; i < order.items.length; i++) {
    const item = order.items[i]
    const book = await bookService(item.bookId)
    total += item.quantity * book.price
  }
  return total
}

const placeOrderService = (order: Order) =>
  calculateAmountService(order).then(
    totalAmount =>
      ({
        success: true,
        totalAmount
      } as PlacedOrderResult)
  )

const processor: AsyncProcessor = (
  orderId: string
): Promise<PlacedOrderResult> => {
  return orderService(orderId)
    .then(validationService)
    .then(placeOrderService)
    .catch(() => {
      return {
        success: false,
        totalAmount: 0.0
      }
    })
}

export default processor
