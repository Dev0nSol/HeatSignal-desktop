from fastapi import FastAPI

app = FastAPI(title="HeatSignal Backend (placeholder)", docs_url=None, openapi_url=None)

VERSION = "0.1.0"

@app.get("/health")
def health():
    return {"ok": True}

@app.get("/version")
def version():
    return {"version": VERSION}
