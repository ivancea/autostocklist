# Based on: https://github.com/avalanche123/cassandra-web
FROM ruby:2

RUN gem install cassandra-web

EXPOSE 3000

# ENTRYPOINT sleep 5s && cassandra-web --hosts $(getent hosts $(echo $HOSTS | sed -e "s/,/ /g") | awk '{ print $1 }' | paste -s -d ',') --port 9042
ENTRYPOINT while :; \
    do \
        sleep 20s \
        && getent hosts $(echo $HOSTS | sed -e "s/,/ /g") | awk '{ print $1 }' | paste -s -d ',' | xargs -I {} cassandra-web --hosts {} --port 9042; \
    done