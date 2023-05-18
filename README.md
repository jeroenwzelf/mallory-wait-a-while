# mallory-wait-a-while

On the internet, there are all sorts of bots carefully crafted to search for any open ports, connect to them, and try to brute-force their way in via commonly known usernames and passwords. A common example is [Hydra](https://www.kali.org/tools/hydra/), is a parallelized login cracker which supports numerous protocols to attack. 

To reduce the speed at which these attackers can harm machines (and the people they serve), machines can arm themselves with [tarpits](https://en.wikipedia.org/wiki/Tarpit_(networking)), services on the computer system that purposefully delays incoming connections. The idea is that, when network abuses such as spamming or broad scanning are less effective, it becomes less attractive to do.

This project, ```mallory-wait-a-while``` aims to be a networking tarpit where any connected Mallory (the name of any malicious attacker, often used in cryptography) has to wait for a while _(read: forever)_ to connect.

## SSH tarpit
This program makes use of a section in [paragraph 4.2 of RFC-4253](https://datatracker.ietf.org/doc/html/rfc4253#section-4.2):
> When the connection has been established, both sides MUST send an identification string.  This identification string MUST be 
>
> ```SSH-protoversion-softwareversion SP comments CR LF```
>
> [...]
> 
> The server MAY send other lines of data before sending the version string.  Each line SHOULD be terminated by a Carriage Return and Line Feed.  Such lines MUST NOT begin with "SSH-", and SHOULD be encoded in ISO-10646 UTF-8 [RFC3629] (language is not specified).  Clients MUST be able to process such lines.  Such lines MAY be silently ignored, or MAY be displayed to the client user.

There is no length specified to the _'other lines of data'_ the server may send to a client. That means that, the server can keep the client busy forever by forever sending _'other lines of data'_, and never actually sending the identification string.

### Sidenote
I mainly wrote this project to learn and practice the Rust programming language, and got inspired while reading [Chapter 20 of the Rust guide (Building a Multithreaded Web Server)](https://doc.rust-lang.org/book/ch20-00-final-project-a-web-server.html).