import { pipe } from 'fp-ts/lib/pipeable'
import { orders, books } from './data'
import { validateOrder, Order, ProcessOrder, PlaceOrderResult } from './api'
import { Either, isLeft } from 'fp-ts/lib/Either'
import { taskEither, chain, map } from 'fp-ts/lib/TaskEither'

const evaluateEither = <T>(ma: Either<Error, T>) => {
  if (isLeft(ma)) {
    throw ma.left
  }
  return ma.right
}

const orderService = (orderId: string) =>
  orders[orderId]
    ? taskEither.of<Error, Order>(orders[orderId])
    : taskEither.throwError<Error, Order>(
        new Error(`Order not found: ${orderId}`)
      )

const validationService = (order: Order) => {
  const r = validateOrder(order)
  if (r.valid) {
    return taskEither.of<Error, Order>(order)
  } else {
    return taskEither.throwError<Error, Order>(
      new Error(`${r.error}`)
    )
  }
}

const calculateAmountService = (order: Order) => {
  let total = 0
  for (let i = 0; i < order.items.length; i++) {
    const item = order.items[i]
    total += item.quantity * books[item.bookId].price
  }
  return taskEither.of<Error, number>(total)
}

const placeOrderService = (order: Order) =>
  pipe(
    calculateAmountService(order),
    map(
      totalAmount =>
        ({
          success: true,
          totalAmount
        } as PlaceOrderResult)
    )
  )

const processOrder: ProcessOrder = (orderId: string) =>
  pipe(
    orderService(orderId),
    chain(validationService),
    chain(placeOrderService)
  )().then(evaluateEither)

export default processOrder
