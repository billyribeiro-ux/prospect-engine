export interface PipelineStage {
	readonly id: string;
	readonly tenantId: string;
	readonly name: string;
	readonly sortOrder: number;
	readonly createdAt: string;
	readonly updatedAt: string;
}

export type ActivityKind = "call" | "email" | "note" | "stage_change";

export interface Activity {
	readonly id: string;
	readonly tenantId: string;
	readonly leadId: string;
	readonly kind: ActivityKind;
	readonly body: string;
	readonly occurredAt: string;
	readonly createdAt: string;
}

export interface Lead {
	readonly id: string;
	readonly tenantId: string;
	readonly businessId: string;
	readonly stageId: string;
	readonly ownerUserId: string | undefined;
	readonly createdAt: string;
	readonly updatedAt: string;
}
