# Ideas for JVM-rs

* Self-modifying code

  What *if* the Rust JVM could **modify** itself during execution?
  Optimise itself, and rewrite itself to create a COMPLETELY different
  program than what was originally written?
  
  *TODO*: I need to find out if LLVM has some support or ways to accomplish this.

  Context: I've just been reading about polymorphic code, and I really
  wonder if that technique could be applied to a virtual machine..
