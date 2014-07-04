extern crate libc;
extern crate native;

extern crate tickit;

#[start]
fn start(argc: int, argv: *const *const u8) -> int {
    native::start(argc, argv, main)
}

fn main()
{
    println!("demo-timer not yet ported");
}

/*
#include "tickit.h"

#include <errno.h>
#include <signal.h>
#include <stdio.h>
#include <string.h>
#include <unistd.h>

int still_running = 1;

static void sigint(int sig)
{
  still_running = 0;
}

int main(int argc, char *argv[])
{
  TickitTerm *tt;

  tt = tickit_term_new();
  if(!tt) {
    fprintf(stderr, "Cannot create TickitTerm - %s\n", strerror(errno));
    return 1;
  }

  tickit_term_set_input_fd(tt, STDIN_FILENO);
  tickit_term_set_output_fd(tt, STDOUT_FILENO);
  const struct timeval await = (const struct timeval){ .tv_sec = 0, .tv_usec = 50000 };
  tickit_term_await_started(tt, &await);

  tickit_term_setctl_int(tt, TICKIT_TERMCTL_ALTSCREEN, 1);
  tickit_term_setctl_int(tt, TICKIT_TERMCTL_CURSORVIS, 0);
  tickit_term_clear(tt);

  signal(SIGINT, sigint);

  int counter = 0;

  while(still_running) {
    struct timeval to = { .tv_sec = 1, .tv_usec = 0 };
    tickit_term_input_wait(tt, &to);

    tickit_term_goto(tt, 5, 5);
    tickit_term_printf(tt, "Counter %d", counter++);
  }

  tickit_term_destroy(tt);

  return 0;
}
*/
