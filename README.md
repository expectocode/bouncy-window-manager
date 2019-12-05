# Bouncy Window Manager

A very silly X window manager that bounces its windows around.

For https://jvns.ca/blog/2019/11/25/challenge--make-a-bouncy-window-manager/. After [the solutions post](https://jvns.ca/blog/2019/12/03/solutions-to-the-tiny-window-manager-challenge/) mentioned there were few non-C solutions, I decided to give it a try in Rust.

I pinky swear I didn't peek at the other solutions before writing mine. I did copy a bit from my other project [hacksaw](https://github.com/neXromancers/hacksaw) though. Which reminds me, I need to clean up that code...

Warning, **lots** of `unwrap` usage. You have been warned.

![GIF with multiple bouncing windows](https://user-images.githubusercontent.com/15344581/70195855-ca508400-16fe-11ea-8d98-8fff96ecf0e3.gif)
![GIF with one bouncing window](https://user-images.githubusercontent.com/15344581/70195786-907f7d80-16fe-11ea-8932-de7938436abf.gif)
