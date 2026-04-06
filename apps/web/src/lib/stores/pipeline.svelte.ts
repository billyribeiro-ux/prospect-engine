import type { Lead, PipelineStage } from "@pe/types/pipeline";

class PipelineState {
	stages = $state<readonly PipelineStage[]>([]);
	leads = $state<readonly Lead[]>([]);
	selectedLeadId = $state<string | undefined>(undefined);

	setStages(next: readonly PipelineStage[]): void {
		this.stages = next;
	}

	setLeads(next: readonly Lead[]): void {
		this.leads = next;
	}

	selectLead(id: string | undefined): void {
		this.selectedLeadId = id;
	}
}

export const pipelineState = new PipelineState();
