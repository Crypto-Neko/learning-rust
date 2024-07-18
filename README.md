# Learning Rust

My goal with this project is to learn the syntax and logic of rust so that I can use it for an upcoming project. Specifically, I've decided to write [NyaaCrypt](https://github.com/Crypto-Neko/nyaacrypt) in rust instead of the C and C++ languages.

To accomplish this, I've had ChatGPT generate an extensive list of challenges for me. Some of them are fairly simple, but I'm going to do them anyway as I think getting some short, quick bursts of practice at the beginning can be helpful.
I should have all of these done in a week or so and then I can get started on NyaaCrypt.

Here are the challenges as generated exactly by ChatGPT. I may change them slightly in the future if they're not optimal for some reason.

### Basic Challenges

1. **Hello World**:
   Write a simple program that prints "Hello, world!" to the console.

2. **Variable Assignment**:
   Create a program that assigns values to variables and prints them.

3. **Basic Arithmetic**:
   Write a program that performs and prints the result of basic arithmetic operations: addition, subtraction, multiplication, and division.

4. **Control Flow - If/Else**:
   Create a program that takes a number as input and prints whether it is positive, negative, or zero.

5. **Loops - For Loop**:
   Write a program that prints the numbers from 1 to 10 using a for loop.

6. **Loops - While Loop**:
   Write a program that prints the numbers from 10 to 1 using a while loop.

7. **Functions**:
   Create a function that takes two integers as parameters and returns their sum. Call this function and print the result.

8. **String Manipulation**:
   Write a program that takes a string as input and prints it in reverse.

9. **Structs**:
   Define a struct to represent a rectangle with width and height fields. Implement a method to calculate the area of the rectangle.

10. **Enums**:
   Define an enum to represent a traffic light with three variants: Red, Yellow, and Green. Write a function that takes a traffic light and returns how long it stays on.

### Intermediate Challenges

11. **Vectors**:
    Create a program that initializes a vector with the first 10 Fibonacci numbers and prints them.

12. **File I/O**:
    Write a program that reads a text file and prints its contents to the console.

13. **Error Handling**:
    Write a function that takes a string and attempts to parse it into an integer, returning a Result type. Handle errors gracefully.

14. **Ownership and Borrowing**:
    Implement a function that takes a reference to a vector and prints its elements without taking ownership.

15. **HashMap**:
    Write a program that counts the occurrences of each word in a given text and prints the results.

16. **Command-Line Arguments**:
    Write a program that accepts command-line arguments and prints them.

17. **Concurrency with Threads**:
    Create a program that spawns multiple threads, each printing a message to the console.

18. **Using External Crates**:
    Write a program that makes an HTTP request to a public API and prints the response. Use the `reqwest` crate.

19. **Pattern Matching**:
    Implement a simple calculator that takes an operator and two operands from the user and performs the operation using pattern matching.

20. **Basic Web Server**:
    Use the `hyper` crate to create a simple web server that responds with "Hello, world!" to any request.

### Advanced Challenges

21. **Concurrent Data Structures**:
    Implement a thread-safe counter using `Arc` and `Mutex`.

22. **Asynchronous Programming**:
    Create an asynchronous program that fetches data from multiple URLs concurrently using the `tokio` crate.

23. **Game Development**:
    Write a simple command-line game, like Tic-Tac-Toe.

24. **Functional Programming**:
    Implement a program that uses iterators and combinators (like `map`, `filter`, `collect`) to process a collection of data.

25. **Custom Iterators**:
    Implement a custom iterator for a custom data structure.

26. **Macros**:
    Write a macro that generates getter and setter methods for a struct.

27. **Lifetime Annotations**:
    Implement a program that uses complex lifetime annotations to manage references.

28. **Multithreaded Web Server**:
    Expand the basic web server to handle multiple requests concurrently.

29. **Embedded Programming**:
    Write a program for a microcontroller using the `embedded-hal` crate.

30. **Network Programming**:
    Create a simple chat server and client using the `tokio` crate.

These challenges will help you get familiar with Rust's syntax, standard library, and unique features like ownership and borrowing. As you progress, you'll also get a taste of more advanced concepts and libraries in the Rust ecosystem.
