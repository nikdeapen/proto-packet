# Issues

## Services

- **Streaming:** The request and response are buffered there isn't really streaming support.
- **Async:** The scope is async with actix-web but the service is not.
- **Comments:** The comments on service calls don't propagate to generated code.
- **Errors:** Error handling is rudimentary at best, it could be better than text.
- **Routing:** All calls use POST and are hard coded paths. Some calls might want GET.
- **Middleware:** There are no middleware hooks on services or service calls.
- **Clients:** There is no client generation, only server generation.
- **Actix Only:** The only supported web-framework is actix-web.
