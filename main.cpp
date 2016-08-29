class nsICSSPseudoElement;

class nsCSSPseudoElements
{
public:
  static nsICSSPseudoElement* after;
  static nsICSSPseudoElement* before;
};

nsICSSPseudoElement* nsCSSPseudoElements::after;
nsICSSPseudoElement* nsCSSPseudoElements::before;

extern "C" void test();

int main()
{
  test();
}
