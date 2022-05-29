---
slug: Launch Announcement
title: Launch Announcement
date: 2022-05-30
authors: [luismbaezco]
tags: [stress-testing, testing, cp-tool, tool]
---

Hi Competitive Programming Community!!

I want to share with the entire competitive programming community my latest project: [Quick Test CLI](https://github.com/LuisMBaezCo/quicktest), which is a multiplatform open source tool to perform stress testing in competitive programming and that was inspired after watching the following screencasts: [1v1 Coding | 2020 Lockout Championship](https://www.youtube.com/watch?v=uABbBGtEWks&t=1434s) and [How to test your solution in Competitive Programming, on Linux?](https://www.youtube.com/watch?v=JXTVOyQpSGM) Made by [Errichto](https://codeforces.com/profile/Errichto) some time ago.

📦**Source Code:** [https://github.com/LuisMBaezCo/quicktest](https://github.com/LuisMBaezCo/quicktest)

📖**Docs:** [https://luismbaezco.github.io/quicktest](https://luismbaezco.github.io/quicktest)

🚀 **Installation:** [https://luismbaezco.github.io/quicktest/docs/getting-started/installation](https://luismbaezco.github.io/quicktest/docs/getting-started/installation)

### What Quick Test CLI provides vs. the traditional way of performing stress testing?

- Multi-platform
- Supports several languages
- Friendly user interface
- You don't have to create or use tedious bash scripts
- It can be a standard for the community
- It is a collaborative project that can continue to grow for the benefit of the competitive programming community.

#### _Quick Test CLI supports several types of tests:_

![main gif](/gif/main.gif)


### `quicktest cmp | qt cmp`


It checks the correctness of the algorithm we want to verify by comparing it with a brute force solution which is usually very slow, but is 100% sure to provide the correct solution.

```shell
quicktest cmp --target-file=main.cpp --correct-file=correct.cpp --gen-file=gen.cpp
# Or shorter:
qt cmp -t main.cpp -c correct.cpp -g gen.cpp
```

![cmp gif](/gif/cmp.gif)

### `quicktest check | qt check`

In some problems more than one answer is accepted, so the quicktest cmp command would not work correctly, in this case a script checker is used to verify the correctness of the algorithm.

```shell
quicktest check --target-file=main.cpp --checker-file=correct.cpp --gen-file=gen.cpp
# Or shorter:
qt check -t main.cpp -c check.cpp -g gen.cpp
```

![check gif](/gif/check.gif)


### `quicktest stress | qt stress`

Verify that the code execution time does not exceed what is allowed, using a random generator for multiple test cases.

Note: In this scenario there is no slower solution with which to compare the correction.

```shell
quicktest stress --target-file=main.cpp --gen-file=gen.cpp
# Or shorter:
qt stress -t main.cpp -g gen.cpp
```

![stress gif](/gif/stress.gif)

### Future Updates

Feature for testing interactive problems is planned to be implemented in future versions

---


I will be grateful to receive feedback, requests for new features and requests for improvements

Don't forget to give it ⭐in the [Github](https://github.com/LuisMBaezCo/quicktest) repository

Thanks!