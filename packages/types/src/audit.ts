export type AuditDimensionId =
	| "performance"
	| "seo"
	| "content"
	| "design"
	| "accessibility"
	| "security"
	| "tech_debt";

export interface DimensionScore {
	readonly dimension: AuditDimensionId;
	readonly score: number;
	readonly weight: number;
	readonly signals: Readonly<Record<string, unknown>>;
}

export interface AuditScore {
	readonly composite: number;
	readonly dimensions: readonly DimensionScore[];
	readonly gradedAt: string;
}

export interface AuditRun {
	readonly id: string;
	readonly tenantId: string;
	readonly businessId: string;
	readonly status: "pending" | "running" | "complete" | "failed";
	readonly score: AuditScore | undefined;
	readonly errorMessage: string | undefined;
	readonly createdAt: string;
	readonly updatedAt: string;
}
