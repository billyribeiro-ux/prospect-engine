export interface WhiteLabelConfig {
	readonly logoUrl: string | undefined;
	readonly primaryColor: string | undefined;
	readonly companyName: string;
}

export interface ReportConfig {
	readonly id: string;
	readonly tenantId: string;
	readonly title: string;
	readonly whiteLabel: WhiteLabelConfig;
	readonly createdAt: string;
	readonly updatedAt: string;
}
