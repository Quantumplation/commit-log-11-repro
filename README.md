# Reproduce Repo

This repo reproduces an apparent issue with the commitlog crate.

The commitlog appears to not be reading an existing log properly.

In this repo, we create a new commit log, print the last offset,
and then print each message in the log.

Then we append the message "Test" to the log, and flush it.

I would expect this to print:

First Run:
```
Offset: None
```
Second Run:
```
Offset: Some(0)
0 - Test
```
Third Run:
```
Offset: Some(1)
0 - Test
1 - Test
```

Instead, it prints:
```
Offset: None
```

Every time the application is run.
