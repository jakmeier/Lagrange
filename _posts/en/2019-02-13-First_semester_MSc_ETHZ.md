---
layout: post
title: "My first master semester at ETHZ"
author: "Jakob Meier"
categories: meta
tags: [study, ETHZ]
image: /19/eth/mugs.jpg
image_tooltip: The coffee mug shelter in the computer science department.
lang: en
ref: msc-start
techs: 
  ant:
    title: Apache Ant
    description: Most commonly used to build Java applications. It uses XML formatted build files and has integrations for all major IDEs. In the above-described ASL course, it is required that the final hand-in builds properly from the command line using the Ant default task.
    url: http://ant.apache.org/
    img: ant.svg
  azure:
    title: Microsoft Azure
    description: Microsoft has offered their cloud computing services for free (up to a limit) for the students of the ASL course and we used this platform to run experiments on a real-life infrastructure.
    img: azure.png
    url: https://azure.microsoft.com/
  bash:
    title: GNU BASH shell scripts
    description: BASH is the standard shell on most UNIX-based operating systems. Writing BASH scripts is one of the most direct ways to automate the execution of a number of sequential shell commands and it also offers loops and conditional branches.
    img: bash.png
  bgl:
    title: Boost graph library (bgl)
    description: The boost libraries are not-quite standard libraries for C++. They are open-source and typically well-maintained. BGL, which we used for the algorithms lab, is part of these libraries and it includes common graph algorithms such as Dijkstra's shortest paths.
    img: boost.png
    url: https://www.boost.org/doc/libs/1_69_0/libs/graph/doc/
  cgal:
    title: The Computational Geometry Algorithms Library (CGAL)
    description: A C++ library that we used for infinite-precision, geometric computations as well as efficient computations of Delaunay triangulations.
    img: cgal.png
    url: https://www.cgal.org/
  c++:
    title: C++11  and its Standard Template Library (STL)
    description: C++ is an object-oriented, low-level programming language and C++11 is the ISO-approved standard version of it from 2011. The STL consists of a bunch of template classes that implement common algorithms and data structures.
    img: cpp.png
  make:
    title: GNU make
    description: A build automation tool. It shines at (re-)generating files that depend on other files. It is often used to build C/C++ applications or LaTeX documents.
    img: gnu.png
    url: https://www.gnu.org/software/make/
  gdb:
    title: The GNU Project Debugger
    description: To debug programs, or to reverse-engineer binaries, gdb is the fundamental tool that allows to freeze the executing process in any state and observe it step-by-step.
    url: https://www.gnu.org/software/gdb/
    img: archer.svg
  gnuplot:
    title: gnuplot
    description: A free, command-line first and versatile data visualization tool that works on most modern platforms. 
    img: gnuplot.png
    url: http://www.gnuplot.info/
  wifi:
    title: IEEE 802.11
    description: The set of protocols commonly referred to as WLAN or Wi-Fi are standardized under this shared family-name. The actual protocols then append one or two additional characters to the end of the name. (e.g. 802.11n)
    img: wifi.png
    url: https://en.wikipedia.org/wiki/IEEE_802.11
  intelsgx:
    title: Intel SGX
    description: An extension to the Intel processor architecture that allows running some code in a trusted enclave that is verifiable from a remote machine, without the need to trust the OS or any of the hardware, besides the CPU.
    img: intelsgx.jpg
    url: https://software.intel.com/en-us/sgx
  java:
    title: Java 8
    description: Java is a high-level, object-oriented programming language that runs in the Java Virtual Machine. Java 8 introduced a number of new features, including some functional approaches such as lambda expressions.
    img: java.png
  memcached:
    title: Memcached
    description: An open-source, distributed key-value store caching system. It is easy to use, yet scales amazingly. Known users of it include Facebook and Wikipedia.
    img: memcached.png
    url: https://memcached.org/
  python:
    title: Python 3
    description: A very popular programming language these days for its simplicity and good ecosystem. I used it here for data-processing because it does not require compilation and can therefore be used easily from make or bash scripts.
    img: python.png
    url: https://www.python.org/
---

<p class="intro">Five months with much coffee, short nights and many amazing new insights that I could learn. Read this to find out which course selection kept me 100% motivated throughout the semester.</p>

After a full year of work as a software engineer, at a [place](https://www.bsi-software.com/en/home.html) that was full of friendly people, interesting challenges and great working conditions, I went back to [ETH Zurich](https://www.ethz.ch) last fall, as it has always been the plan. 

Although I was a bit sad to leave my former employer, my motivation for the coming master studies in computer science was at an extra high level. So I actually had a hard time to select only as many courses as would fit in my schedule. After hours of trying to figure out which courses fit me best and talking to friends who have already taken some of them, I started my semester with 44 ECTS credits, 14 more than recommended. 

At the same time, I had been employed as a teaching assistant in the new 5th-semester bachelor's course *Computer Systems* for which I had to prepare and hold weekly exercise sessions for a group of undergraduate students.

Over the course of the semester, I found myself forced to drop some courses in order to profit fully from the remaining courses that required already all of my capacity. So I have finished the semester with 33 ECTS and I really have learned a ton while attending the selected courses.

In this article, I want to summarize briefly each of the courses I visited for the entire semester. I hope to give the reader a short insight into some ETHZ courses and potentially help some future students to decide on their own subject choice.

###  List of all courses 
{:.no_toc} 
* TOC
{:toc}

![Image: The main building of ETH Zurich.](/assets/img/19/eth/ETHZ.JPG)

## Advanced Systems Lab
This course requires good time management skills and students are asked to think for themselves more than in any other course I have taken so far.

In the first week, a project description is released and the date for the hand-in of the final report is published. Some tutorials introduce students to basic queueing theory during the first third of the semester. Other than that, there is no direct teaching in this course. It is clear from the very beginning, this is a masters-level course, we expect students to be professional in their work and have no need for a babysitter. 

The book [The Art of Computer Systems Performance Analysis](https://www.amazon.com/Art-Computer-Systems-Performance-Analysis/dp/0471503363) is key to learn about the theory required in the course. For the practical skills, it seemed to me that the organizer expects the students to be familiar in large parts with basic socket networking concepts, with build-tools such as **Apache Ant** and with common utilities to communicate with remote machines and run scripts on them. But I think it should be possible to learn most of that also during the project. Students will have to learn all of it by themselves but there are enough resources out there on the web to get along.

The exact task varies from year to year. This time, each student had to implement a middleware in **Java 8** that can communicate with standard **Memcached** clients and servers. The middleware also had to gather statistical information in real-time and report it in a reasonable way. Many design choices are left to the students. The following image illustrates a high-level view of the design I ended up implementing for this project.

![Image: ASL design overview.](/assets/img/19/eth/asl.png)

This implementation is about 20% of the total workload. The actual core of the project is a series of experiments around it, using **Azure** cloud machines. And, of course, a rigorous report of everything with statistical analysis and tests behind each and every claim made. The grading is all about the scientific procedure and the line of action taken and documented.

While the experiments have to follow a strict structure, everyone is free to choose their own tools to execute, automate and evaluate the experiments. In my case, I have used **Bash**-scripts to run the software with the correct parameters on the different machines. I then used a combination of **Python 3**, **gnuplot** and **Make** for my evaluations, analysis and data presentation.

Below is an example graph from my report that has been produced by *gnuplot* using a series of *make* rules that defined how the raw output files of many machines and repetitions needed to be combined and processed. 

![Image: ASL example graph.](/assets/img/19/eth/asl_graph.png)

## Algorithms Lab
The algorithms lab in a nutshell: Competitive programming in C++.

Weekly tutorial sessions introduce general concepts and new exercises are released each week that can be submitted to an online judge that offers live feedback. Non-optimal, yet correct, solutions only grant a fraction of the full points and optimality usually is dependent on the input data.

A typical exercise combines several techniques from dynamic programming, graph algorithms, geometric algorithms, and linear-/ quadratic programming. And in order to map the problem properly, solid knowledge in geometry, graph theory, and various combinatorics is required.

In total, there are around 70 practical exercises and only a fraction of the students is able to solve them all before the date of the exam. This is also visible on the online scoreboard where students can compete against each other, although this scoreboard has no influence on the final grade.

The exam takes place on two different days, each day offers 3 new exercises and 6 hours time to solve them, again with live feedback. 

To get any points at all for a problem, often it is necessary to come up with a novel idea to map the problem. From the numerous students who fail this course each year, I would say most of them fail at that stage because no amount of preparation can guarantee that one will come up with all the right ideas during the exam.

Then, to actually have enough time to start work on all exercise during the exams, students must have a lot of experience programming with **C++ 11**, its **STL**, the **Boost graph library (BGL)** and the **Computational Geometry Algorithms Library (CGAL)**. 

Needless to say, to achieve full points for an exam exercise, the optimal algorithm has to be found and the implementation must be on point.

Despite the difficulty of the course, even my friend who did not pass the course had to admit that solving these problems is very satisfying and fun, at least once the solution is found.

## System Security
Probably the number one introduction course to become a hacker. Or simply a nice fit for any security concerned computer scientists. Basic cryptography and systems preknowledge are strongly recommended.

The course features many general topics on a theoretical level such as side-channel attacks, IOT security or platform security. Usually, all of the covered topics are directly linked to recent publications on the field and students are supposed to read them. In the case of simple power analysis attacks, there is also a short lab in which students read out a private RSA key from an embedded device. (The entire setup with an oscilloscope is done by a teaching assistant.)

Then there are some modern technologies such as **Intel SGX** and attacks such as [Meltdown](https://meltdownattack.com/) and [Foreshadow](https://foreshadowattack.eu/) discussed in greater detail but all without practical exercises. 

The practical exercises cover the simplest attacks covered in the course, namely exploiting buffer-overflow vulnerabilities and [return-oriented programming](https://en.wikipedia.org/wiki/Return-oriented_programming) to escalate privileges. Already knowing **gdb** beforehand certainly helps to get these exercises done quickly but I think it's also possible to pick that knowledge up while solving the exercises.


## Wireless Networking and Mobile Computing
This course is basically a one-man show by [Stefan Mangold](https://www.lovefield.ch/~smangold) who has been working and researching in the field for years. He worked on various wireless standards and has been one of the main authors on the [IEEE 802.11e standard](https://www.cs.ccu.edu.tw/~yschen/course/93-1/2.pdf) which introduces quality of service (QoS) for Wi-Fi.

While the **IEEE 802.11** protocols are the main focus, a number of other wireless technologies are covered in class as well. And for most topics students will have to perform and evaluate experiments, either with simulations or with physical devices.

For example, we have implemented a chat app that uses serial communicates to an [Arduino board](https://www.arduino.cc/) that then uses a single LED per client to send and receive visible light signals from other Arduino clients.

![Image: The visible light communication experiment setup.](/assets/img/19/eth/led_communication.jpg)

About every other week, students have to hand in a written report including the results, the collected data, and some data visualizations. Here is one example graph from the visual-light communication experiments.

![Graph: Visible light communication performance for different distances and package sizes.](/assets/img/19/eth/led_communication_graph.png)

## System Construction
This course is all about understanding systems completely, from alpha to omega.

The lecture explains the challenges that we have to face when building a system and possible solutions. And then there are in-class exercises, which are well supervised by the lecturer himself, in which students can apply those concepts by adding some missing pieces to the mostly complete code that is provided.

In contrast to a typical OS course, in this one, students will always see (and hopefully understand) the entirety of the code base required to, say, boot a system, or to schedule tasks. This is possible because the course relies on minimal and very simple OS designs that have been built over the years surrounding the [Project Oberon](http://www.projectoberon.com/). 

Students taking this course should not be afraid to code in an unfamiliar programming language, namely **Oberon**, which has a syntax that deviates from what most of today's students are probably used to. But it also has some very nice features that, personally, I would love to see in modern languages.

## Informal Methods
If a company requires new software, they typically don't bother to prove the code to be correct. In practice, they simply cannot afford to do so. Typically, a test-framework offers a much better risk-vs-cost tradeoff.

Now, if people really want to prove some code to be correct, then they reach out to formal method tools, for instance, [Isabelle/Isar](https://www.ethz.ch/content/dam/ethz/special-interest/infk/inst-infsec/information-security-group-dam/research/publications/pub2007/Isabelle_Isar.pdf). The pure formal parts are then fully automated by these frameworks.

While those tools can usually automate the *formal* parts of a proof, there are still *informal* steps that have to be done manually. One good example is to find and express a loop-invariant that is both strong enough and also weak enough that it is self-preserving and sufficient to perform the whole proof.

This course focuses on practicing exactly that, looking at code and solving the difficult parts of the proofs, those that cannot be automated. And these techniques are also useful when arguing about code *without* reaching out to a full-fledged formal framework.

Indeed the course also works fine for students without experience in formal methods and the exercises never require to use any formal method tools. Simple but exact annotations in the form of comments between the code line are all that the lecturer asks for.

In my opinion, the skills taught here are useful for any professional programmer. It opens a way of thinking about code that helps to prevent and detecting bugs simply by looking at the code. 

#  Final words
{:.no_toc} 
I tried to keep the above descriptions on the objective side. But I must emphasize that I thoroughly enjoyed all of the above courses and I really want to thank all the organizers! 

Speaking of the workload this semester, I should also admit that it was not easy to follow all the courses with full commitment. Especially the combination of the two labs, which are both *very* time-consuming, with all the project deadlines of the other courses left me little room to breathe during the semester. But somehow, with enough coffee and even more motivation, I managed to finish all these courses without major compromises.