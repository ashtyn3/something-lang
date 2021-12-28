#include <iostream>
#include <memory>
#include <ostream>
#include <string>

// struct str_lit {
//   int length;
//   string chs;
//   string display() { return chs; }
//   str_lit(string str) : chs(str)
// };

struct INT32_LIT {
  int_fast32_t num;
  INT32_LIT(int_fast32_t i) : num(i){};
};

INT32_LIT S32_ADD(INT32_LIT x, INT32_LIT y) { return x.num + y.num; }
INT32_LIT S32_MUL(INT32_LIT x, INT32_LIT y) { return x.num * y.num; }

int main() {
  // std::unique_ptr<INT32_LIT> answer(
  //     new INT32_LIT(S32_ADD(*my_ptr, INT32_LIT(2))));
  //
  // std::unique_ptr<INT32_LIT> mul_answer(
  //     new INT32_LIT(S32_MUL(*answer, INT32_LIT(2))));

  // std::unique_ptr<INT32_LIT> answer_1 = std::make_unique<INT32_LIT>(
  //     INT32_LIT(S32_ADD(INT32_LIT(2), INT32_LIT(2))));

  std::unique_ptr<INT32_LIT> answer_1 =
      std::make_unique<INT32_LIT>(INT32_LIT(4));
  std::cout << (*answer_1).num << std::endl;
  return 0;
}
