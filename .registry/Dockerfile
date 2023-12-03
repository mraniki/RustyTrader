FROM rust:1.67


##
# Environment variables
#
ENV OP_PATH=/usr/bin/op
ENV OP_SERVICE_ACCOUNT_TOKEN=None

##
# Install 1Password CLI
#
RUN wget -P /tmp/ https://downloads.1password.com/linux/debian/$(dpkg --print-architecture)/stable/1password-cli-$(dpkg --print-architecture)-latest.deb
RUN dpkg -i /tmp/1password-cli-$(dpkg --print-architecture)-latest.deb
RUN op --version

##
# Deploy app
#
COPY . /app
RUN cargo install --path /app

##
# Healthcheck and PORT
#
EXPOSE 8080
HEALTHCHECK --interval=5s CMD curl --fail http://localhost:8080 || exit 1

##
# Run app
#
CMD [ "/app/rustic_trader" ]



# RUN apt-get update && \
# apt-get -y install chromium && \
#   pip install -U pip && \
#   pip install playwright && \
#   playwright install chromium && playwright install-deps && \
#   rm -rf /var/lib/apt/lists/*
