#include <memory>

extern "C" {

// remove when yer done
#include <stdexcept>
[[noreturn]] void todo(const char* message) {
   throw std::runtime_error(message);
}

// cpp has a `std::optional` buuuut, ffi doesn't support it :<
struct Opt {
   bool has_value;
   int  value;
};

// also couldnt do generics (templates) for the same reason
class Stack {
   private:
      struct Node {
         int data;
         std::unique_ptr<Node> next = nullptr;

         Node(int data) : data(data) {}
      };

      std::unique_ptr<Node> top = nullptr;
      uint64_t len = 0;

   public:
      Stack() {}

      void push(int data) {
         todo("Implement push");
      }

      Opt pop() {
         todo("Implement pop");
      }

      Opt peek() const {
         return is_empty() // mmmm, ternary! :p shame rust doesnt have it
            ? Opt {false, 0}
            : Opt {true, top->data};
      }

      inline uint64_t get_len() const {
         return len;
      }
};


// kinda stupid that i have to do this, cpp bad? err.. cpp ffi bad? :/
Stack* stack_new() { return new Stack(); }
void stack_delete(Stack* stack) { delete stack; }
void stack_push(Stack* stack, int data) { stack->push(data); }
Opt stack_pop(Stack* stack) { return stack->pop(); }
Opt stack_peek(Stack* stack) { return stack->peek(); }
uint64_t stack_get_len(Stack* stack) { return stack->get_len(); }

}
