# ipv4calc 🛰️

**ipv4calc** is a simple CLI application that can analyze ipv4 addresses and calculate subnets.

# Installation ⏬

Install the **latest release** from the **Release** section.

# Usage 💻

- After you have installed the executable, open a terminal under the folder where the executable is located.
- To get the details from an ip address just type it like this:
  
  ```bash
  ipv4calc 192.168.0.1/24
  ```
  if the prefix is omitted, it will be the class one.
- To calculate the subnets you have to type the ip followed by the number of subnet you want:

  ```bash
  ipv4calc 192.168.0.1 2
  ```
  if you want to view the details of each subnet, type `-v` as the last parameter.
  
