# fastapi-template

Minimal FastAPI service skeleton. Part of the AEGIS repo template.

## Run locally

```bash
python -m venv .venv
source .venv/bin/activate   # .venv\Scripts\activate on Windows
pip install -e .
uvicorn app.main:app --reload
```

Then visit [http://localhost:8000/](http://localhost:8000/) for the root response and [http://localhost:8000/docs](http://localhost:8000/docs) for the auto-generated OpenAPI UI.

## Status

Vanilla starting point. AEGIS conventions (settings, logging, auth, middleware, deployment config) will be layered in during the normalization work tracked by [aegis-initiative#30](https://github.com/aegis-initiative/aegis-initiative/issues/30).
