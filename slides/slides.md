#the Cost of Abstractions

#### Massimiliano Mantione
#### Gianluca Carucci

---

###once upon a time...

![](assets/partenza.jpg)

^
this story starts with me and Massi take the train from milan to Lecco reaching our Hyperfair office.
Often I tell to Massi about my journey in functional programming structure like Monad, ADT and so on... and how them helps me to make my code more readable checked and powerfull.

---

###What is the Cost of Abstractions? 
 
![](assets/lecco.jpg)

^
The strange thing is all trip ends in the same way, with Massi that ask me "Cool, but what about the cost of those abstractions?"
My answer is always the same....

---

###ehm....
![150%](assets/ehm.png)

^
ehm... I dont know, but it's the perfect moment to have a talk together about this topic!

---

[slide presentazione massi]

---

[slide presentazione gianluca]

---

###what **cost** means?

^
domanda al pubblico...

---

#it's not all about performances

---
###typescript -> javascript

```javascript
  const order = await orderService(orderId)
  if (order == null) {
    return {
      success: false
    }
  }
  const validationResult = await validationService(order)
  if (!validationResult.valid) {
    return placedOrderFailed
  }
  return await placeOrderService(order)

```
---

## typescript

- **no performace penalty**

---

## typescript

- no performace penalty
- **cognitive overhead**

---

##performaces matter
#### let's go to measure 

![](assets/performance.jpg)

---


###Simple use case:
- **create an order of books**

---

###Simple use case:
- create an order of books
- **validate the order**

---

###Simple use case:
- create an order of books
- validate the order
- **place the order**

---
##benchmark

- "buy a book" use case
- run [xxx] iterations
- with [yyy]% of failure orders

---

[slide with qr code where download slides&code]

---

#ready 
#steady 
#go!

![](assets/go.jpg)

---

###async typescript

```javascript
  const order = await orderService(orderId)
  if (order == null) {
    return {
      success: false
    }
  }
  const validationResult = await validationService(order)
  if (!validationResult.valid) {
    return placedOrderFailed
  }
  return await placeOrderService(order)
```

---

###let's add an abstraction layer (fp-ts)

^
we add an abstraction layer using fp-ts in order to make code readable and simplify (task) composition

---

###functional javascript

```javascript
  return pipe(
    orderService(orderId),
    chain(validationService),
    chain(placeOrderService)
  )
```
---
#RESULTS
vanilla typescript 20s

---
#RESULTS
vanilla typescript 20s
functional typescript 12s

---

#WTF???

---
#RESULTS
vanilla typescript 20s
functional typescript 12s

---

#RESULTS
vanilla target es3 typescript 20s
functional typescript 12s

---

#RESULTS
vanilla target es3 typescript 20s
vanilla target es6 typescript 5s
functional typescript 12s



---
###LESSION LEARNED
#### remember to check default compiler options ->

####same code different perfomance depending on compiler option

---

### let's enforce business rules at compile time

---

###checked functional javascript

```javascript

function validationService (o: Order): Validated<Order>  {
  const r = validateOrder(order)
  if (r.valid) {
    return valid<Order>(order)
  } else {
    return notvalid(`${r.error}`)
  }
}


```

---

###checked functional javascript

```javascript

function calculateAmountService (order: Valid<Order>) {
  return pipe(
    order.right.items.map(item =>
      pipe(
        bookService(item.bookId),
        map(b => b.price * item.quantity)
      )
    ),
    array.sequence(taskEither),
    map(amounts => {
      return amounts.reduce((a, b) => a + b, 0)
    })
  )
}
function placeOrderService (order: Valid<Order>) {
  return pipe(
    calculateAmountService(order),
    map(placedOrderSuccess)
  )
}

```
---

###checked functional javascript

```javascript

  return pipe(
    orderId,
    orderService,
    map(validationService),
    chain(mapTask(placeOrderService))
  )
```

---

#RESULTS
vanilla target es3 typescript 20s
vanilla target es6 typescript 5s
functional typescript 12s
**checked functional typescript 12.1s**

---

#RESULTS
- no performance penalty
- cognitive overhead

---

## can we take best of the two worlds (vanilla + checked)

---

#and now something completely different

---

#what happen if we change language?


---
#RUST
###zero ~~cost~~ overhead abstraction

---

...

---

...

---

#what about the web?

^
what about the web? are we forced to pay for abstractio

---

###WebAssembly
####same code base different host

---
#RESULT

rust native [x]s
**rust wasm [y]s**

---

vogliamo software migliore

---

ma cosa significa migliore?

---

profit = revenue * time_to_market - cost
time to market: [0...1]

migliore profit massimo

---

come lo massimiziamo?


---

performance vs mantenibilità

---

possiamo ottenere entrambe?

---

no perchè ogni astrazione incide positivamente o sulle performance o sulla mantenibilità, non su entrambe

---

#one more thing
come incide l'overhead cognitivo?

---

