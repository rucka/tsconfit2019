![](assets/cover.jpg)

---
<br>
#once upon a time...

![](assets/partenza.jpg)

^
this story starts with me and Massi take the train from milan to Lecco reaching our Hyperfair office.
Often I tell to Massi about my journey in functional programming structure like Monad, ADT and so on... and how them helps me to make my code more readable checked and powerfull.

---

<br>
#What is the Cost of Abstractions? 
 
![](assets/lecco.jpg)

^
The strange thing is all trip ends in the same way, with Massi that ask me "Cool, but what about the cost of those abstractions?"
My answer is always the same....

---
![150%](assets/ehm.png)

^
ehm... I dont know, but it's the perfect moment to have a talk together about this topic!


---
![](assets/legend.jpg)

---

![](assets/massimiliano.jpg)

---

![](assets/gianluca.jpg)

---
<br>
#what **cost** means?
![](assets/cost.jpg)

^
domanda al pubblico...

---

<br>
#it's not all about performances

![](assets/performance_m.jpg)


---
### javascript

```javascript
const processor = async (orderId) => {
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
}
```

![](assets/bg_g.jpg)

---

###typescript

```javascript
const processor: AsyncProcessor = async (
  orderId: string
): Promise<PlacedOrderResult> => {
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
}
```
![](assets/bg_g.jpg)

---



## typescript

- **no performace penalty**

![](assets/bg_m.jpg)

---

## typescript

- no performace penalty
- **cognitive overhead**

![](assets/bg_g.jpg)

---

##performaces matter
#### let's go to measure 

![](assets/performance_m.jpg)

---


###"buy a book" use case
- **create an order of books**

![](assets/usecase.jpg)

---

###"buy a book" use case
- create an order of books
- **validate the order**

![](assets/usecase.jpg)

---

###"buy a book" use case
- create an order of books
- validate the order
- **place the order**

![](assets/usecase.jpg)

---
##benchmark

- **"buy a book" use case**

![](assets/benchmark_m.jpg)

---
##benchmark

- "buy a book" use case
- **[xxx] different orders**

![](assets/benchmark_m.jpg)

---
##benchmark

- "buy a book" use case
- [xxx] different orders
- **[yyy]% orders fail**

![](assets/benchmark_m.jpg)

---
##benchmark

- "buy a book" use case
- [xxx] different orders
- [yyy]% orders fail
- **[zzz] iterations**

![](assets/benchmark_m.jpg)

---

![original](assets/qrcode.jpg)

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
![](assets/bg_m.jpg)

---

#RESULTS
...

![](assets/result_m.jpg)


---

#RESULTS
**async typescript 20s**

![](assets/result_m.jpg)

---

#next step
####**add** an abstraction layer (fp-ts)

![](assets/step_g.jpg)

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

![](assets/bg_g.jpg)

---
#RESULTS
async typescript 20s

![](assets/result_m.jpg)

---
#RESULTS
async typescript 20s
**functional typescript 12s**

![](assets/result_m.jpg)

---
<br>
#sounds strange
![fit](assets/wtf_m.jpg)

---
#RESULTS
async typescript 20s
functional typescript 12s

![](assets/result_m.jpg)

---

#RESULTS
**async typescript(_target es3_) 20s**
functional typescript 12s

![](assets/result_m.jpg)

---

#RESULTS
async typescript(_target es3_) 20s
**async typescript(_target es6_) 5s**
functional typescript 12s

![](assets/result_m.jpg)

---

![](assets/lesson_m.jpg)

---

- **same code could have (huge) different perfomance**

![](assets/lesson_m.jpg)

---

- same code could have (huge) different perfomance
- **check default compiler options**

![](assets/lesson_m.jpg)

---
#next step
#### **enforce** business rules at compile time

![](assets/step_g.jpg)

---

###checked functional javascript

```javascript
type NotValid = Left<Error>
type Valid<A> = Right<A>
type Validated<A> = Either<Error, A>

function validationService (o: Order): Validated<Order>  {
  const r = validateOrder(order)
  if (r.valid) {
    return valid<Order>(order)
  } else {
    return notvalid(`${r.error}`)
  }
}


```
![](assets/bg_g.jpg)

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
![](assets/bg_g.jpg)

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
![](assets/bg_g.jpg)

---

#RESULTS
async typescript(_target es3_) 20s
async typescript(_target es6_) 5s
functional typescript 12s

![](assets/result_m.jpg)

---

#RESULTS
async typescript(_target es3_) 20s
async typescript(_target es6_) 5s
functional typescript 12s
**checked functional typescript 12.1s**

![](assets/result_m.jpg)

---

#RESULTS
- no performance penalty
- cognitive overhead

![](assets/result_g.jpg)

---

###how achive 
###the best of 
###the two world?
![](assets/magic_m.jpg)

^
performance and maintenability

---

![fit](assets/montypythons_m.jpg)

---

#next step
#### **change** language

![](assets/step_m.jpg)

---

![fit](assets/rust_m.jpg)

---

[TBD]

![](assets/bg_m.jpg)

---

[TBD]

![](assets/bg_m.jpg)

---

#RESULT
async typescript(_target es3_) 20s
async typescript(_target es6_) 5s
functional typescript 12s
checked functional typescript 12.1s

![](assets/result_m.jpg)


---

#RESULT
async typescript(_target es3_) 20s
async typescript(_target es6_) 5s
functional typescript 12s
checked functional typescript 12.1s
**rust native [x]s**

![](assets/result_m.jpg)


---
<br>
#what about the web?
![](assets/web_m.jpg)

^
what about the web? are we forced to pay for abstraction?

---

#next step
#### same code from native **to WebAssembly** 
![](assets/step_m.jpg)

---
#RESULT
async typescript(_target es3_) 20s
async typescript(_target es6_) 5s
functional typescript 12s
checked functional typescript 12.1s
rust native [x]s

![](assets/result_m.jpg)

---

#RESULT
async typescript(_target es3_) 20s
async typescript(_target es6_) 5s
functional typescript 12s
checked functional typescript 12.1s
rust native [x]s
**rust wasm [y]s**

![](assets/result_m.jpg)

---

![](assets/summary_g.jpg)

---

- **we're focused on better software**

![](assets/summary_g.jpg)

---

- we're focused on better software
- **but what "better" software means?**

![](assets/summary_g.jpg)

---

$$profit(t) = (revenue(t)*time2market)-cost(t), 
\quad\quad \text{[$0>=time2market<=1$]}$$

![](assets/summary_g.jpg)

---

- we're focused on better software
- but what "better" software means?
- **$$"better" = max(profit(t))$$**

![](assets/summary_g.jpg)

---

- we're focused on better software
- but what "better" software means?
- $$"better" = max(profit(t))$$
- **how do that?**

![](assets/summary_g.jpg)

---

performance 
_vs_
maintainability

![](assets/streetfighter.jpg)

---

#design abstraction
- :-1: performance 
- :+1: maintainability

![](assets/summary_g.jpg)

---

#code optimisation abstraction
- :+1: performance 
- :-1: maintainability

![](assets/summary_m.jpg)

---

#but...

![fit](assets/homer_doubt_g.jpg)

---

###can we reach **both** performance and maintainability?

![fit](assets/homer_doubt_g.jpg)

---

###**rust** is the answer!
####zero ~~cost~~ overhead abstraction
![fit](assets/homer_yeah_m.jpg)

---

#but...

![fit](assets/homer_doubt_g.jpg)

---

###what about **cognitive overhead**?

![fit](assets/homer_doh_g.jpg)

---
#summary
- cost have different shapes
- abstractions have different shapes
- each decision **hides** a cost
- there are **no** zero cost abstraction
- choose abstraction depending on the **context**

![](assets/summary.jpg)

---

![](assets/thank.jpg)

---

##questions?
![](assets/question.jpg)

---

![original](assets/qrcode.jpg)
