FROM ubuntu:latest
RUN apt-get update
RUN apt-get install -y bird
RUN mkdir -p /run/bird/
COPY bird.conf /etc/bird/bird.conf

EXPOSE 179/tcp

CMD /usr/sbin/bird -c /etc/bird/bird.conf -f