void dummy_fn(void) {
  return;
}

int main(void) {
  // Fine
  dummy_fn();

  // Not fine
  void (*fn_ptr)(void) = dummy_fn;
  fn_ptr();

  return 0;
}
