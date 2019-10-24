theme: Ostrich, 4

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

![](assets/lecco.jpg)


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

^
we'll analyse the cost of abstractions under two points of view: performance and manteinability

---

![](assets/gianluca.jpg)

^
[G]

---

![](assets/massimiliano.jpg)

^
[M]

---
<br>
#what does **cost** mean?
![](assets/cost.jpg)

^
ask to the audience...

---

<br>
#it's not all about performance

![](assets/performance_m.jpg)


---

##consider **typescript**
###as _abstraction_ 
##over **javascript**


---

###compile from typescript

```javascript
const processor: AsyncProcessor = async (
  orderId: string): Promise<PlacedOrderResult> => {
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

###to javascript

```javascript
const processor                 = async (
  orderId        )                             => {
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

##code is _not modified_
##types are **stripped**

---

#it's not all about performance

- **no performace penalty**

![](assets/bg_m.jpg)

---

#it's not all about performance

- no performace penalty
- **you will pay a *build time* cost**


![](assets/bg_g.jpg)

---

#it's not all about performance

- no performace penalty
- you will pay a *build time* cost
- **cognitive overhead**

![](assets/bg_g.jpg)

---

#cognitive overhead

**cognitive**: something we should *know*
**overhead**: an *extra* effort

---

#cognitive overhead

**cognitive**: something we should *know*
**overhead**: an *extra* effort

the **extra** learning effort **the whole team** must spend!

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
- **compute and place the order**

![](assets/usecase.jpg)

---
##benchmark

- **"buy a book" use case**

![](assets/benchmark_m.jpg)

---
##benchmark

- "buy a book" use case
- **process 500k orders**

![](assets/benchmark_m.jpg)

---
##benchmark

- "buy a book" use case
- process 500k orders
- **5% failure rate**

![](assets/benchmark_m.jpg)

---
##benchmark

- "buy a book" use case
- process 500k orders
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

#Async
...

![](assets/result_m.jpg)


---

#Async
**async typescript 6.772μs**

![](assets/result_m.jpg)

---

#next step
####**add** an abstraction layer (fp-ts)

![](assets/step_g.jpg)

^
we add an abstraction layer using fp-ts in order to make the code more readable and to simplify (task) composition

---

###functional typescript

```javascript
  return pipe(
    orderService(orderId),
    chain(validationService),
    chain(placeOrderService)
  )
```

![](assets/bg_g.jpg)

---
#from Async to FP
async typescript 6.772μs

![](assets/result_m.jpg)

---
#from Async to FP
async typescript 6.772μs
**functional typescript 5.952μs**

![](assets/result_m.jpg)

---
<br>
#this sounds strange
![fit](assets/wtf_m.jpg)

---
#from Async to FP
async typescript 6.772μs
functional typescript 5.952μs

![](assets/result_m.jpg)

---

#from Async to FP
async typescript(**target es3**) 6.772μs
functional typescript(**target es3**) 5.952μs

![](assets/result_m.jpg)

---

#from Async to FP (ES3->ES2018)
async typescript(_target es3_) 6.772μs
async typescript(**target es2018**) **2.004μs**
functional typescript(_target es3_) 5.952μs
functional typescript(**target es2018**) **5.636μs**

![](assets/result_m.jpg)

---

![](assets/lesson_m.jpg)

---

- **the same code can run with (hugely) different perfomance**

![](assets/lesson_m.jpg)

---

- the same code can run with (hugely) different perfomance
- **the compiler is part of the abstraction implementation**

![](assets/lesson_m.jpg)

---
#next step
#### **enforce** business rules at compile time

![](assets/step_g.jpg)

---

###checked functional typescript

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

###checked functional typescript

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

###checked functional typescript

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

#from FP to FP-Checked
async typescript 2.004μs
functional typescript 5.636μs

![](assets/result_m.jpg)

---

#from FP to FP-Checked
async typescript 2.004μs
functional typescript 5.636μs
**checked functional typescript 5.402μs**

![](assets/result_m.jpg)

---

#from FP to FP-Checked
- no performance penalty
- slightly *faster* (one less `chain`?)
- cognitive overhead

![](assets/result_g.jpg)

---

![original](assets/typescript-simple.png)

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

![](assets/bg_m.jpg)

---

# What does *zero cost* mean?
- *zero cost* for the abstractions you *do not use*
- what you *do* use, cannot be done *any better*
- this means "zero *runtime overhead*"

![](assets/bg_m.jpg)

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
- **faithful to the Typescript one**

![](assets/bg_m.jpg)

---

# A Rust implementation
- faithful to the Typescript one
- **line by line adaptation**

![](assets/bg_m.jpg)


---

# A Rust implementation
- faithful to the Typescript one
- line by line adaptation
- **let's benchmark!**

![](assets/bg_m.jpg)

---

#from ts to rust
async typescript 2.004μs

![](assets/result_m.jpg)

---

#from ts to rust
async typescript 2.004μs
**what do you expect?**

![](assets/result_m.jpg)

---

#from ts to rust
async typescript 2.004μs
**async rust (native) 0.2410μs**

![](assets/result_m.jpg)

---

#from ts to rust
async typescript 2.004μs
async rust (native) 0.2410μs
**and 8x speedup?**

![](assets/result_m.jpg)

---

# WAT?
**does this make sense?**

![](assets/bg_m.jpg)

---

# WAT?
does this make sense?
**is it a fair comparison?**

![](assets/bg_m.jpg)

---

# WAT?
does this make sense?
is it a fair comparison?
**let's investigate**

![](assets/bg_m.jpg)

---

# WAT?
does this make sense?
is it a fair comparison?
let's investigate
**we start from scratch**

![](assets/bg_m.jpg)

---

# WAT?
does this make sense?
is it a fair comparison?
let's investigate
we start from scratch
**with a synchronous typescript version**

![](assets/bg_m.jpg)

---

# Start from Scratch
**from *synchronous* typescript version**

![](assets/bg_m.jpg)


---

# Start from Scratch
from *synchronous* typescript version
**then we add minimal abstractions**

![](assets/bg_m.jpg)


---

# Start from Scratch
from *synchronous* typescript version
then we add minimal abstractions
**one by one**

![](assets/bg_m.jpg)


---

# Start from Scratch
from *synchronous* typescript version
then we add minimal abstractions
one by one
**and benchmark each step**

![](assets/bg_m.jpg)

---

# Abstraction Steps

![](assets/bg_m.jpg)

---

# Abstraction Steps
***synchronous* typescript**

![](assets/bg_m.jpg)

---

# Abstraction Steps
*synchronous* typescript
**synchronous *functional* typescript**

![](assets/bg_m.jpg)

---

# Abstraction Steps
*synchronous* typescript
synchronous *functional* typescript
**synchronous *callbacks***

![](assets/bg_m.jpg)

---

# Abstraction Steps
*synchronous* typescript
synchronous *functional* typescript
synchronous *callbacks*
***nextTick* callbacks**

![](assets/bg_m.jpg)

---

# Abstraction Steps
*synchronous* typescript
synchronous *functional* typescript
synchronous *callbacks*
*nextTick* callbacks
**promises**

![](assets/bg_m.jpg)

---

# Abstraction Steps
*synchronous* typescript
synchronous *functional* typescript
synchronous *callbacks*
*nextTick* callbacks
promises
**async await**

![](assets/bg_m.jpg)

---

# Abstraction Steps
*synchronous* typescript
synchronous *functional* typescript
synchronous *callbacks*
*nextTick* callbacks
promises
async await
***functional* tasks**

![](assets/bg_m.jpg)

---

# Abstraction Steps
*synchronous* typescript
synchronous *functional* typescript
synchronous *callbacks*
*nextTick* callbacks
promises
async await
*functional* tasks
***checked* types**

![](assets/bg_m.jpg)

---

#Typescript Abstractions

![original](assets/typescript-full.png)

---

#Typescript Abstractions
plain Typescript is *fast*

![original](assets/typescript-full-plain.png)

---

#Typescript Abstractions
plain Typescript is *fast*
abstractions built on it are *slow*

![original](assets/typescript-full-abstract.png)

---

#Typescript Abstractions
plain Typescript is *fast*
abstractions built on it are *slow*
the nodejs event loop does *not* help much

![original](assets/typescript-full-event-loop.png)

---

#Typescript Abstractions
plain Typescript is *fast*
abstractions built on it are *slow*
the nodejs event loop does *not* help much
**let's do the same with Rust...**

![original](assets/typescript-full.png)

---

### rust, plain

```rust
pub async fn process(order_id: &String) -> Result<f64, ()> {
    match order_service(order_id).await {
        Some(order) => {
            let validation = validation_service(&order).await;
            match validation {
                Ok(order) => match place_order_service(order).await {
                    Ok(res) => Ok(res.amount),
                    Err(_) => Err(()),
                },
                _ => Err(()),
            }
        }
        _ => Err(()),
    }
}
```

![](assets/bg_m.jpg)

---

### rust, idiomatic

```rust
pub async fn process(order_id: &String) -> Result<f64, ()> {
    let order = order_service(order_id).await;
    let validated = validation_service(order).await.map_err(|_| ())?;
    Ok(place_order_service(validated).await.map_err(|_| ())?.amount)
}
```

![](assets/bg_m.jpg)

---

### rust, composable

```rust
pub fn process(order_id: &'static String) ->
        impl Future<Output = Result<f64, ()>> {
    compose!(
        order_service,
        validation_service,
        place_order_service,
        map_order_amount
    )(order_id)
}
```

![](assets/bg_m.jpg)


---

#We have seen rust...

Should we measure it?

![](assets/result_m.jpg)

---

![original](assets/typescript-rust.png)

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

![original](assets/typescript-wasm.png)

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

$$profit = revenue - cost$$

![](assets/summary_g.jpg)

---

$$profit = (revenue*time2market)-cost$$

###where
$$\quad\quad \text{[$0>=time2market<=1$]}$$


![](assets/summary_g.jpg)

---

$$profit(t) = (revenue(t)*time2market(t))-cost(t)$$ 
###where
$$\quad\quad \text{[$0>=time2market<=1$]}$$

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

###we know how to achieve 
###__no__ performance __cost__ 
###and still have maintainability...

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

#wrapping up
there are different kinds of **abstractions**

![](assets/summary.jpg)

---

#wrapping up
there are different kinds of **abstractions**
and there are different kinds of **costs**

![](assets/summary.jpg)

---

#wrapping up
there are different kinds of **abstractions**
and there are different kinds of **costs**
different abstractions **involve** different costs

![](assets/summary.jpg)

---

#wrapping up
there are different kinds of **abstractions**
and there are different kinds of **costs**
different abstractions **involve** different costs
there are **no** zero cost abstractions, but...

![](assets/summary.jpg)

---

#wrapping up
there are different kinds of **abstractions**
and there are different kinds of **costs**
different abstractions **involve** different costs
there are **no** zero cost abstractions, but...
...we can choose **where** to incur costs!

![](assets/summary.jpg)

---

#wrapping up
there are different kinds of **abstractions**
and there are different kinds of **costs**
different abstractions **involve** different costs
there are **no** zero cost abstractions, but...
...we can choose **where** to incur costs!
**how do we choose?**

![](assets/summary.jpg)

---

![original](assets/typescript-rust-points.png)

---

![original](assets/typescript-rust-tunnel.png)

---

![original](assets/typescript-rust-tunnel-ts.png)

---

![original](assets/typescript-rust-tunnel-fp.png)

---

![original](assets/typescript-rust-tunnel-promise.png)

---

![original](assets/typescript-rust-tunnel-async.png)

---

![original](assets/typescript-rust-context.png)

---

![original](assets/typescript-rust-context-fp.png)

---

![original](assets/typescript-rust-context-ts-async.png)

---

![original](assets/typescript-rust-context-rust.png)

---

![](assets/thank.jpg)

---

##questions?
![](assets/question.jpg)

---

![original](assets/qrcode.jpg)
