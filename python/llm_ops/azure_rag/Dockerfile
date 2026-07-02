# Use a lightweight Python base image
FROM python:3.10-slim

COPY --from=ghcr.io/astral-sh/uv:latest /uv /bin/uv

WORKDIR /webapp

COPY pyproject.toml uv.lock ./

RUN uv sync --frozen --no-dev

COPY webapp/ .

ENV PATH="/webapp/.venv/bin:$PATH"

ENTRYPOINT [ "uvicorn" ]

CMD [ "--host", "0.0.0.0", "main:app" ]
