"""Module that implements the producer."""
# pylint: disable=too-few-public-methods
import logging
from time import sleep
from typing import Generator
# pylint: disable=no-name-in-module
from generator import twenty_digits_code
from common.message_broker import MessageBroker, RabbitMQ

LOGGER = logging.getLogger('producer')

# Duration of interval between sends
INTERVAL_SEC = 5


class Producer():
    """
    Implementation of a producer which generates elements
    and publishes to a message broker.
    """
    def __init__(self, message_broker: MessageBroker, generator: Generator) -> None:
        self._message_broker = message_broker
        self._generator = generator

    def start(self) -> None:
        """Main loop which queues random 20 digits alphanumerical strings."""
        LOGGER.error("Starting production!")
        for body in self._generator:
            self._message_broker.publish(body=body)
            LOGGER.error("Sent %s", body)
            sleep(INTERVAL_SEC)

if __name__ == '__main__':
    producer = Producer(
        message_broker=RabbitMQ(
            service='producer'
        ),
        generator=twenty_digits_code(),
    )
    producer.start()
