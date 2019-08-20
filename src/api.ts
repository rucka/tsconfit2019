import { books } from './data'

export type Book = { name: string; author: string; price: number }
export type Order = { date: Date; items: OrderLine[] }
export type OrderLine = { bookId: string; quantity: number }

export type EvaluateOrder = (orderId: string) => Promise<number>
//service1 get order
//service2 validate order
//if not valid show error
//if valid map order to calculate total amount
//place the order

export type OrderNotValid = 'NoItems' | 'BookNotExists'
export function validateOrder(
  order: Order
): { valid: true } | { valid: false; error: OrderNotValid } {
  const invalid = (error: OrderNotValid) => ({ valid: false, error })
  if (order.items.length === 0) {
    return invalid('NoItems')
  }
  for (let i = 0; i < order.items.length; i++) {
    if (!books[order.items[i].bookId]) {
      return invalid('BookNotExists')
    }
  }
  return { valid: true }
}
