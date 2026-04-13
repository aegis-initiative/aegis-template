from fastapi import FastAPI

app = FastAPI(title="FastAPI Template")


@app.get("/")
def root() -> dict[str, str]:
    return {"status": "ok", "service": "fastapi-template"}


@app.get("/health")
def health() -> dict[str, str]:
    return {"status": "healthy"}
