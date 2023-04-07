# Overview

The following was discussed on https://habilis.net/cronic/ does.

## The problem with running cron jobs

cron sends automatic emails when a cron job writes anything 

- to stdout
- to stderr

But it doesn't care if a cron job exited with an error code.


## What do we want?

We want cron to send a mail

- if an error happened in a cron job
- (perhaps) if something got written to stderr by the cron job


# Possible solution

Running a wrapper which invokes the cron job and takes care to trigger sending email by cron.

This is exactly what the shell script provided by https://habilis.net/cronic/ does.


# rcronic

Instead of using the script provided by Chuck Houpt I wrote a small Rust program which serves as a wrapper to 
run a cron job.

## How does `rcronic` work?

If the cron job returns with an error or if `--stderr` was specified and the cron job wrote something to stderr 
then `rconic` writes the cron jobs' error code and stderr to stdout so that a mail gets sent.

# Strict error handling in bash/zsh scripts

It is highly recommended to turn on strict error handling in bash/zsh scripts which is done by

```sh
set -eu
```

If required also tracing could be turned on

```sh
set -x
```










