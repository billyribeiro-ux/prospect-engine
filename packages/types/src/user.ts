export interface Tenant {
	readonly id: string;
	readonly name: string;
	readonly createdAt: string;
	readonly updatedAt: string;
}

export interface User {
	readonly id: string;
	readonly tenantId: string;
	readonly email: string;
	readonly displayName: string;
	readonly createdAt: string;
	readonly updatedAt: string;
}

export interface Session {
	readonly userId: string;
	readonly tenantId: string;
	readonly accessTokenExpiresAt: string;
}
