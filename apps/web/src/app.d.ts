import type { ThemeId } from "@pe/types/theme";
import type { DensityMode } from "@pe/types/theme";

declare global {
	namespace App {
		interface Locals {
			theme: ThemeId;
			density: DensityMode;
		}
	}
}

export {};
