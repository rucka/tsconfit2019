![](assets/cover.jpg)

---
<br>
#once upon a time...

![](assets/partenza.jpg)

^
This story starts with me and Massi traveling by train from Milan to Lecco to reach our Hyperfair office.
Often I tell Massi about my adventures in functional programming structures like Monad, ADT and so on... and how them help me making my code more readable, safe and powerful.

---

<br>
#What is the Cost of Abstractions? 
 
![](assets/lecco.jpg)

^
The strange thing is every trip ends in the same way, with Massi asking me "Cool, but what about the cost of those abstractions?"
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
#what does **cost** mean?
![](assets/cost.jpg)

^
domanda al pubblico...

---

<br>
#it's not all about performance

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

##performace matters
#### let's measure 

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
- **500k different orders**

![](assets/benchmark_m.jpg)

---
##benchmark

- "buy a book" use case
- 500k different orders
- **5% failure rate**

![](assets/benchmark_m.jpg)

---
##benchmark

- "buy a book" use case
- 500k different orders
- 5% failure rate
- **measure mean order time**

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
**async typescript 6.772μs**

![](assets/result_m.jpg)

---

#next step
####**add** an abstraction layer (fp-ts)

![](assets/step_g.jpg)

^
we add an abstraction layer using fp-ts in order to make the code more readable and to simplify (task) composition

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
async typescript 6.772μs

![](assets/result_m.jpg)

---
#RESULTS
async typescript 6.772μs
**functional typescript 5.952μs**

![](assets/result_m.jpg)

---
<br>
#this sounds strange
![fit](assets/wtf_m.jpg)

---
#RESULTS
async typescript 6.772μs
functional typescript 5.952μs

![](assets/result_m.jpg)

---

#RESULTS
**async typescript(_target es3_) 6.772μs**
functional typescript(_target es3_) 5.952μs

![](assets/result_m.jpg)

---

#RESULTS
async typescript(_target es3_) 6.772μs
**async typescript(_target es2018_) 2.004μs**
functional typescript(_target es3_) 5.952μs
**functional typescript(_target es2018_) 5.636μs**

![](assets/result_m.jpg)

---

![](assets/lesson_m.jpg)

---

- **the same code can run with (hugely) different perfomance**

![](assets/lesson_m.jpg)

---

- the same code can run with (hugely) different perfomance
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
async typescript 2.004μs
functional typescript 5.636μs

![](assets/result_m.jpg)

---

#RESULTS
async typescript 2.004μs
functional typescript 5.636μs
**checked functional typescript 5.402μs**

![](assets/result_m.jpg)

---

#RESULTS
Show typescript-simple.png

![](assets/result_m.jpg)

---

#RESULTS
- no performance penalty
- slightly *faster* (one less `chain`?)
- cognitive overhead

![](assets/result_g.jpg)

---

###can we have 
###the best of 
###both worlds?
![](assets/magic_m.jpg)

^
performance and maintenability

---

![fit](assets/montypythons_m.jpg)

---

#next step
#### a **different** language

![](assets/step_m.jpg)

---

![fit](assets/rust_m.jpg)

---

# What does *zero cost* mean?
- *zero cost* for the abstractions you *do not use*
- what you *do* use, cannot be done *any better*
- this means "zero *runtime overhead*"
- you will pay a *build time* cost 
- plus cognitive overhead...

![](assets/bg_m.jpg)

---

# A Rust implementation
- faithful to the Typescript one
- line by line adaptation
- let's benchmark!

![](assets/bg_m.jpg)

---

#RESULTS
async typescript 2.004μs

![](assets/result_m.jpg)

---

#RESULTS
async typescript 2.004μs
**what do you expect?**

![](assets/result_m.jpg)

---

#RESULTS
async typescript 2.004μs
**async rust (native) 0.2410μs**

![](assets/result_m.jpg)

---

#RESULTS
async typescript 2.004μs
async rust (native) 0.2410μs
**and 8x speedup?**

![](assets/result_m.jpg)

---

# WAT?
does this make sense?
is it a fair comparison?
let's investigate
we start from scratch
with a synchronous typescript version

![](assets/bg_m.jpg)

---

# Start from Scratch
from *synchronous* typescript version
then we add minimal abstractions
one by one
and benchmark each step

![](assets/bg_m.jpg)

---

#RESULTS
Show typescript-full.png
(briefly describe each abstraction step)

![](assets/bg_m.jpg)

---

#What Happened?
plain Typescript is *fast*
abstractions built on it are *slow*
the nodejs event loop does *not* help much
let's do the same with Rust...

![](assets/bg_m.jpg)

---

#RESULTS
Show typescript-rust.png

![](assets/bg_m.jpg)

---
<br>
#what about the web?
![](assets/web_m.jpg)

^
what about the web? are we *forced* to pay for abstractions?

---

#next step
#### same Rust code running on **WebAssembly** 
![](assets/step_m.jpg)

---
#RESULTS
Show typescript-wasm.png

![](assets/result_m.jpg)

---

![](assets/summary_g.jpg)

---

- **we're focusing on better software**

![](assets/summary_g.jpg)

---

- we're focusing on better software
- **but what does "better" software mean?**

![](assets/summary_g.jpg)

---

$$profit(t) = (revenue(t)*time2market)-cost(t), 
\quad\quad \text{[$0>=time2market<=1$]}$$

![](assets/summary_g.jpg)

---

- we're focusing on better software
- but what does "better" software mean?
- **$$"better" = max(profit(t))$$**

![](assets/summary_g.jpg)

---

- we're focusing on better software
- but what does "better" software mean?
- $$"better" = max(profit(t))$$
- **how do we do that?**

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

###can we achieve **both** performance and maintainability?

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
- costs have different shapes
- abstractions have different shapes
- design decisions **involve** a costs
- there are **no** zero cost abstractions, but...
- ...we can choose **where** to incur costs
- choose abstractions depending on the **context**

![](assets/summary.jpg)

---

![](assets/thank.jpg)

---

##questions?
![](assets/question.jpg)

---

![original](assets/qrcode.jpg)
