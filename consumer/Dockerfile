FROM python:3.7.1

RUN python -m venv /venv

COPY ./consumer/requirements.txt /usr/src/app/requirements.txt
COPY ./common/requirements.txt /usr/src/app/common/requirements.txt

WORKDIR /usr/src/app/

RUN /venv/bin/pip install -r requirements.txt -r common/requirements.txt

COPY consumer consumer
COPY common common
COPY .pylintrc ./

RUN /venv/bin/pip install --no-cache ./consumer ./common

RUN /venv/bin/mypy --ignore-missing-imports ./consumer
RUN /venv/bin/pylint ./consumer/consumer/*.py
RUN /venv/bin/pytest
