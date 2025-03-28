# BambangShop Publisher App
Tutorial and Example for Advanced Programming 2024 - Faculty of Computer Science, Universitas Indonesia

---

## About this Project
In this repository, we have provided you a REST (REpresentational State Transfer) API project using Rocket web framework.

This project consists of four modules:
1.  `controller`: this module contains handler functions used to receive request and send responses.
    In Model-View-Controller (MVC) pattern, this is the Controller part.
2.  `model`: this module contains structs that serve as data containers.
    In MVC pattern, this is the Model part.
3.  `service`: this module contains structs with business logic methods.
    In MVC pattern, this is also the Model part.
4.  `repository`: this module contains structs that serve as databases and methods to access the databases.
    You can use methods of the struct to get list of objects, or operating an object (create, read, update, delete).

This repository provides a basic functionality that makes BambangShop work: ability to create, read, and delete `Product`s.
This repository already contains a functioning `Product` model, repository, service, and controllers that you can try right away.

As this is an Observer Design Pattern tutorial repository, you need to implement another feature: `Notification`.
This feature will notify creation, promotion, and deletion of a product, to external subscribers that are interested of a certain product type.
The subscribers are another Rocket instances, so the notification will be sent using HTTP POST request to each subscriber's `receive notification` address.

## API Documentations

You can download the Postman Collection JSON here: https://ristek.link/AdvProgWeek7Postman

After you download the Postman Collection, you can try the endpoints inside "BambangShop Publisher" folder.
This Postman collection also contains endpoints that you need to implement later on (the `Notification` feature).

Postman is an installable client that you can use to test web endpoints using HTTP request.
You can also make automated functional testing scripts for REST API projects using this client.
You can install Postman via this website: https://www.postman.com/downloads/

## How to Run in Development Environment
1.  Set up environment variables first by creating `.env` file.
    Here is the example of `.env` file:
    ```bash
    APP_INSTANCE_ROOT_URL="http://localhost:8000"
    ```
    Here are the details of each environment variable:
    | variable              | type   | description                                                |
    |-----------------------|--------|------------------------------------------------------------|
    | APP_INSTANCE_ROOT_URL | string | URL address where this publisher instance can be accessed. |
2.  Use `cargo run` to run this app.
    (You might want to use `cargo check` if you only need to verify your work without running the app.)

## Mandatory Checklists (Publisher)
-   [ ] Clone https://gitlab.com/ichlaffterlalu/bambangshop to a new repository.
-   **STAGE 1: Implement models and repositories**
    -   [ ] Commit: `Create Subscriber model struct.`
    -   [ ] Commit: `Create Notification model struct.`
    -   [ ] Commit: `Create Subscriber database and Subscriber repository struct skeleton.`
    -   [ ] Commit: `Implement add function in Subscriber repository.`
    -   [ ] Commit: `Implement list_all function in Subscriber repository.`
    -   [ ] Commit: `Implement delete function in Subscriber repository.`
    -   [ ] Write answers of your learning module's "Reflection Publisher-1" questions in this README.
-   **STAGE 2: Implement services and controllers**
    -   [ ] Commit: `Create Notification service struct skeleton.`
    -   [ ] Commit: `Implement subscribe function in Notification service.`
    -   [ ] Commit: `Implement subscribe function in Notification controller.`
    -   [ ] Commit: `Implement unsubscribe function in Notification service.`
    -   [ ] Commit: `Implement unsubscribe function in Notification controller.`
    -   [ ] Write answers of your learning module's "Reflection Publisher-2" questions in this README.
-   **STAGE 3: Implement notification mechanism**
    -   [ ] Commit: `Implement update method in Subscriber model to send notification HTTP requests.`
    -   [ ] Commit: `Implement notify function in Notification service to notify each Subscriber.`
    -   [ ] Commit: `Implement publish function in Program service and Program controller.`
    -   [ ] Commit: `Edit Product service methods to call notify after create/delete.`
    -   [ ] Write answers of your learning module's "Reflection Publisher-3" questions in this README.

## Your Reflections
This is the place for you to write reflections:

### Mandatory (Publisher) Reflections

#### Reflection Publisher-1
Here are the questions for this reflection:
1. In the Observer pattern diagram explained by the Head First Design Pattern book, Subscriber is defined as an interface. Explain based on your understanding of Observer design patterns, do we still need an interface (or trait in Rust) in this BambangShop case, or a single Model struct is enough?

In the classic Observer pattern, an interface or trait is typically used to define a common contract for subscribers. This allows for:
- Polymorphic behavior
- Loose coupling between publishers and subscribers
- Flexibility to add new types of subscribers

However, in BambangShop, the current implementation only has one type of subscriber and doesn't seem to expect differing types, so a trait is not strictly necessary yet.

Although, with Rust's strong type system and potential future extensions, using a trait would be better, as it provides more flexibility and follows the Observer pattern's core principle of defining a consistent interface for observers.

2. id in Program and url in Subscriber is intended to be unique. Explain based on your understanding, is using Vec (list) sufficient or using DashMap (map/dictionary) like we currently use is necessary for this case?

`Vec` is a simple, basic collection type, but unfortunately, it has O(n) lookup time, uniqueness has to be checked manually, and are not thread-safe by default.

Using `DashMap` is necessary because it is a lot more advantageous for this case, as it has:
- O(1) lookup time
- Built-in thread-safety
- Automatic handling of unique keys
- Concurrent access without explicit locking

3. When programming using Rust, we are enforced by rigorous compiler constraints to make a thread-safe program. In the case of the List of Subscribers (SUBSCRIBERS) static variable, we used the DashMap external library for thread safe HashMap. Explain based on your understanding of design patterns, do we still need DashMap or we can implement Singleton pattern instead?
- 
In Rust, thread safety is crucial. Let's analyze the options:

Singleton Pattern Considerations:
- Ensures single global instance
- Requires careful thread-safe implementation
- Can use `lazy_static` or `once_cell` for thread-safe initialization

`DashMap` Advantages:
- Explicitly designed for concurrent access
- Built-in thread-safety
- No manual locking required
- Optimized for multi-threaded scenarios

Recommendation: Continue using `DashMap` because:
- It's specifically designed for concurrent scenarios
- Provides more robust thread-safety
- Reduces manual synchronization complexity
- Aligns with Rust's ownership and concurrency principles

#### Reflection Publisher-2
Here are the questions for this reflection:
1. In the Model-View Controller (MVC) compound pattern, there is no “Service” and “Repository”. Model in MVC covers both data storage and business logic. Explain based on your understanding of design principles, why we need to separate “Service” and “Repository” from a Model?

We need to separate Service and Repository from Model because of the S in SOLID, which is Single Responsibility Principle.

With Service and Repository, the responsibilities are clear:
- Model: Data representation
- Repository: Data persistence
- Service: Business logic and coordination

Separating Service and Repository also makes the code more maintainable as each class is small and focused, as well as easier to understand and modify. 

2. What happens if we only use the Model? Explain your imagination on how the interactions between each model (Program, Subscriber, Notification) affect the code complexity for each model?

Potential complications of using only Model:
- Tight coupling: Models become interdependent
- Reduced flexibility: Difficult to modify individual components
- Complex error handling: Intricate error management
- Performance overhead: Each model carries unnecessary logic
- Violation of Single Responsibility Principle

Interaction Complexities:
- Increased interdependencies
- Difficult to trace logic flow
- Harder to maintain and extend
- Reduced code readability
- Complex state management

3. Have you explored more about Postman? Tell us how this tool helps you to test your current work. You might want to also list which features in Postman you are interested in or feel like it is helpful to help your Group Project or any of your future software engineering projects.

So far I have only used Postman to test the HTTP methods in this project. It is quite convenient that it allows us to send different types of HTTP requests and have format variations for the request body. I have not tried much other features. However, I have heard it has helpful features for group projects, such as mock server creation, automated testing, and automatic API documentation.

#### Reflection Publisher-3
Here are the questions for this reflection:
1. Observer Pattern has two variations: Push model (publisher pushes data to subscribers) and Pull model (subscribers pull data from publisher). In this tutorial case, which variation of Observer Pattern that we use?

In the current implementation, we're using the Push Model of the Observer Pattern.
Key Characteristics of Push Model:
- The publisher (BambangShop) actively sends notifications to subscribers
- Notification details are directly transmitted to subscribers
- Subscribers receive complete information in the update method
- Proactive approach where the publisher controls information flow

2. What are the advantages and disadvantages of using the other variation of Observer Pattern for this tutorial case? (example: if you answer Q1 with Push, then imagine if we used Pull)

Pull Model Characteristics:
- Subscribers request information from the publisher
- Publisher provides minimal notification
- Subscribers responsible for fetching complete details

Advantages of Pull Model:
- Reduced initial data transmission
- Lower memory overhead
- More control for subscribers
- Flexibility in data retrieval
- Potential bandwidth savings

Disadvantages of Pull Model:
- Increased complexity
- More network requests
- Higher latency
- More responsibility on subscriber
- Potential synchronization challenges
- Less efficient data transfer
- Increased system load

3. Explain what will happen to the program if we decide to not use multi-threading in the notification process.

Implications of Non-Threaded Approach:

Performance Bottlenecks
- Notifications processed sequentially
- Slow subscribers block entire notification chain
- Increased response time


Scalability Limitations
- Cannot handle many subscribers efficiently
- Single-threaded execution limits concurrent processing


Reliability Concerns
- Failure in one notification breaks entire process
- No parallel error handling
- Increased risk of system unresponsiveness