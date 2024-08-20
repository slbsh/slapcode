#include <memory>

extern "C" {

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
         auto new_node = std::make_unique<Node>(data);
         if (top) new_node->next = std::move(top);
         top = std::move(new_node);
         ++len;
      }

      Opt pop() {
         if (is_empty()) return Opt {false, 0};

         int value = top->data;
         top = std::move(top->next);
         len--;
         return Opt {true, value};
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
