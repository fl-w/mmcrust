int plus(int a, int b) { return a + b; }

int do(function f, int a, int b) { return f(a, b); }

int main(void) {
  int x = 2;
  int y = 3;
  int times(int a, int b) { return a * b; }

  print_int(do(plus, x, y));
  print_int(do(times, x, y));
  return 0;
}

int fact(int n) {
  int inner_fact(int n, int a) {
    if (n == 0) {
      print_str("found a");
      return a;
    }
    print_str("im hereee ");
    print_int(n);

    return inner_fact(n - 1, a * n);
  }

  return inner_fact(n, 1);
}

int min(int a, int b) {
  if (a < b) {
    return a;
  } else {
    return b;
  }
}

function cplus(int a) {
  int cplusa(int b) { return a + b; }
  return cplusa;
}

function twice(function f) {
  int g(int x) { return f(f(x)); }
  return g;
}
