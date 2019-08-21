import { pipe } from 'fp-ts/lib/pipeable'
import { array } from 'fp-ts/lib/Array'
import { Either, isLeft } from 'fp-ts/lib/Either'
import { taskEither, chain, map } from 'fp-ts/lib/TaskEither'
import { orders, books } from './data'
import {
  validateOrder,
  Order,
  ProcessOrder,
  PlaceOrderResult,
  Book
} from './api'

const evaluateEither = <T>(ma: Either<Error, T>) => {
  if (isLeft(ma)) {
    throw ma.left
  }
  return ma.right
}

const bookService = (bookId: string) =>
  books[bookId]
    ? taskEither.of<Error, Book>(books[bookId])
    : taskEither.throwError<Error, Book>(new Error(`Book not found: ${bookId}`))

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
    return taskEither.throwError<Error, Order>(new Error(`${r.error}`))
  }
}

const calculateAmountService = (order: Order) => {
  const booksAmountTask = order.items.map(item =>
    map<Book, number>(b => b.price * item.quantity)(bookService(item.bookId))
  )
  return map<number[], number>(amounts => {
    return amounts.reduce((a, b) => a + b, 0)
  })(array.sequence(taskEither)(booksAmountTask))
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
