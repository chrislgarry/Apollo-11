FROM ubuntu:16.04
MAINTAINER Jim Lawton

RUN apt-get -y update
RUN apt-get -y upgrade

RUN apt-get -y install git
RUN apt-get -y install build-essential
RUN apt-get -y install make
RUN apt-get -y install g++
RUN apt-get -y install python
RUN apt-get -y install libncurses5
RUN apt-get -y install libncurses5-dev

RUN git clone https://github.com/rburkey2005/virtualagc

# Use this to build a copy of the current directory. 
RUN mkdir /apollo-11
COPY . /apollo-11

# Build virtulagc tools.
RUN cd virtualagc && make clean
RUN cd virtualagc && make yaLEMAP yaAGC yaAGS yaYUL missions

# Use virtualagc to build Apollo-11 source.
RUN cd apollo-11 && VIRTUALAGC=/virtualagc make all
