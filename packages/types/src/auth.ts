/** Shapes returned by `POST /api/v1/auth/{register,login,refresh}` (Rust `AuthSuccess`). */

export interface AuthUser {
	readonly id: string;
	readonly email: string;
}

export interface AuthSuccess {
	readonly token: string;
	readonly refresh_token: string;
	/** Access token lifetime in seconds (JWT `exp` window). */
	readonly expires_in: number;
	readonly user: AuthUser;
}
