FROM ubuntu:latest

RUN apt update -y
RUN apt upgrade -y
RUN apt install build-essential git cargo rustc -y
RUN git clone https://www.github.com/PEDROTUBER44/InstallDotFiles.git
RUN mkdir /home/user/
RUN mv InstallDotFiles/ /home/user/