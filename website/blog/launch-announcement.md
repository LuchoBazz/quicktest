---
slug: Launch Announcement
title: Launch Announcement
date: 2022-05-30
authors: [LuchoBazz]
tags: [stress-testing, testing, cp-tool, tool]
---

Hi, Competitive Programming community!

I want to share my latest project with the competitive programming community: [Quick Test CLI](https://github.com/LuchoBazz/quicktest). It is a multi-platform, open-source tool for stress testing in competitive programming. I got the idea after watching these screencasts by [Errichto](https://codeforces.com/profile/Errichto): [1v1 Coding | 2020 Lockout Championship](https://www.youtube.com/watch?v=uABbBGtEWks&t=1434s) and [How to test your solution in Competitive Programming, on Linux?](https://www.youtube.com/watch?v=JXTVOyQpSGM).

📦**Source Code:** [https://github.com/LuchoBazz/quicktest](https://github.com/LuchoBazz/quicktest)

📖**Docs:** [https://luchobazz.github.io/quicktest](https://luchobazz.github.io/quicktest)

🚀 **Installation:** [https://luchobazz.github.io/quicktest/docs/getting-started/installation](https://luchobazz.github.io/quicktest/docs/getting-started/installation)

### What does Quick Test CLI provide compared to the traditional way of stress testing?

- Multi-platform
- Supports several languages
- Friendly user interface
- You don't need to create or use long bash scripts
- It can become a standard for the community
- It is a collaborative project that can keep growing for the benefit of the competitive programming community

#### _Quick Test CLI supports several types of tests:_

![main gif](https://luchobazz.github.io/quicktest/assets/images/main-ae92a1400e70a721b7e82c2cbc321538.gif)


### `quicktest cmp | qt cmp`


It checks if the algorithm is correct by comparing it with a brute force solution. The brute force solution is usually very slow, but it should always give the correct answer.

```shell
quicktest cmp --target-file=main.cpp --correct-file=correct.cpp --gen-file=gen.cpp
# Or shorter:
qt cmp -t main.cpp -c correct.cpp -g gen.cpp
```

![cmp gif](https://luchobazz.github.io/quicktest/assets/images/cmp-bede9089330a526710da91805355db68.gif)

### `quicktest check | qt check`

In some problems, more than one answer is accepted. In that case, the `quicktest cmp` command may not work well. Instead, you can use a checker script to verify the solution.

```shell
quicktest check --target-file=main.cpp --checker-file=correct.cpp --gen-file=gen.cpp
# Or shorter:
qt check -t main.cpp -c check.cpp -g gen.cpp
```

![check gif](https://luchobazz.github.io/quicktest/assets/images/check-e947f099ca199e2c4fa30a6208980be5.gif)


### `quicktest stress | qt stress`

Checks that the running time does not exceed the time limit, using a random generator for many test cases.

Note: In this scenario, there is no (slow) correct solution to compare against.

```shell
quicktest stress --target-file=main.cpp --gen-file=gen.cpp
# Or shorter:
qt stress -t main.cpp -g gen.cpp
```

![stress gif](https://luchobazz.github.io/quicktest/assets/images/stress-5b3f4114f5d04d7541ee4714c4115da5.gif)

### Future Updates

A feature for testing interactive problems is planned for future versions.

---


I would be grateful for feedback, feature requests, and suggestions for improvements.

Don't forget to give it a ⭐ on the [GitHub](https://github.com/LuchoBazz/quicktest) repository.

Thanks!
