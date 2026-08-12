# Vesper ++

[Vesper](https://www.github.com/ppmpreetham/vesper) but faster, written for speed.
![Vesper Logo](readme/logo.png)

> [!WARNING]  
> This tool is intended for educational purposes only. Use responsibly and ethically. The author is not responsible for any misuse or illegal activities.

### Perf patches

- without tokio multithread: About 18 results (65.7834781s)
- without nable: About 17 results (27.7284363s)
- with nable: About 18 results (25.7736461s)
- with hickory: About 20 results (25.1231852s)
- with compression and without hickory: About 21 results (21.025818s)
- with compression and hickory: About 22 results (19.461094s)
- after stream stopping: About 24 results (19.2897324s)
