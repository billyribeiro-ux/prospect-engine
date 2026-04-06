import type { AuditRun } from "@pe/types/audit";

class AuditState {
	activeAuditId = $state<string | undefined>(undefined);
	recentRuns = $state<readonly AuditRun[]>([]);

	setActiveAudit(id: string | undefined): void {
		this.activeAuditId = id;
	}

	setRecentRuns(runs: readonly AuditRun[]): void {
		this.recentRuns = runs;
	}
}

export const auditState = new AuditState();
