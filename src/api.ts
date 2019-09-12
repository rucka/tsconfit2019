import { books } from './data'

export type Book = { name: string; author: string; price: number }
export type Order = { date: Date; items: OrderLine[] }
export type OrderLine = { bookId: string; quantity: number }

export type PlaceOrderResult = {
  success: true
  totalAmount: number
}
export type ProcessOrder = (orderId: string) => Promise<PlaceOrderResult>
export type Processor = (orderId: string) => Promise<void>

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
