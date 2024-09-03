/ This repository has been moved to codeberg. \
\ https://codeberg.org/ManfredLotz/rcronic    /
 ---------------------------------------------
        \   ^__^
         \  (oo)\_______
            (__)\       )\/\
                ||----w |
                ||     ||

# Overview

The problem with cron is that it creates _unwanted emailed output_. This is described in 
more detail in the next section.

The `rcronic` tool tries to help by running a cron job in a way that an email is sent 
only in case the cron job has failed.



## The problem with running cron jobs

cron sends automatic emails when a cron job writes anything 

- to stdout or
- to stderr

But it doesn't care if a cron job exits with an error code.

This was discussed in more detail on https://habilis.net/cronic/.

## What does rcronic do?

rcronic is a wrapper for a cronjob and creates output only if

- if the cron job exits with an exit code <> 0
- and optionally if something got written to stderr by the cron job
    - this could be necessary if a cron job writes error to stderr but does not return
      with an exit code <> 0
- additionally it can write to a log file if required

# rcronic

If the cron job returns with an error or if `--stderr` was specified and the cron job wrote something to stderr 
then `rconic` writes the cron jobs' error code, stdout and stderr so that a mail gets sent.

# Strict error handling in bash/zsh scripts

It is highly recommended to turn on strict error handling in bash/zsh scripts which is done by

```sh
set -euo pipefail
```

If required also tracing could be turned on

```sh
set -x
```










