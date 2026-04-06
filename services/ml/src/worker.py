"""Background worker stub: polls for inference jobs (future: Redis/queue integration)."""

from __future__ import annotations

import asyncio
import logging
import os

log = logging.getLogger("pe_ml.worker")


async def run_worker_loop(interval_sec: float = 30.0) -> None:
	"""Idle loop until training/inference contracts are implemented."""
	log.info("worker started (stub), interval=%ss", interval_sec)
	while True:
		if os.environ.get("PE_ML_WORKER_EXIT", "").lower() in ("1", "true", "yes"):
			log.info("PE_ML_WORKER_EXIT set; exiting")
			return
		await asyncio.sleep(interval_sec)


def main() -> None:
	logging.basicConfig(level=logging.INFO)
	asyncio.run(run_worker_loop())


if __name__ == "__main__":
	main()
