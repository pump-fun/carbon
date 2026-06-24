# carbon-pump-agent-payments-decoder

Carbon decoder for the pump **tokenized agent payments** program (`AgenTMiC2hvxGebTsgmsD4HHBa8WEcqGFf87iwRRxLo7`).

## Regenerate

Mainnet on-chain IDL is not published; source IDL is vendored from `@pump-fun/agent-payments-sdk@3.0.3` as `../pump_agent_payments.json`.

```bash
pnpm --filter @sevenlabs-hq/carbon-codama-renderer build
pnpm --filter @sevenlabs-hq/carbon-cli build
node packages/cli/dist/cli.js parse \
  --idl ./decoders/pump_agent_payments.json \
  --out-dir ./decoders/pump-agent-payments-decoder
```

Then restore workspace `Cargo.toml` settings (version `0.12.0`, `{ workspace = true }` deps) if the generator overwrote them.
