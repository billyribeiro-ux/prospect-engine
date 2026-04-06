import {
	PRIMARY_PANE_SPLIT_DEFAULT_PERCENT,
	PRIMARY_PANE_SPLIT_MAX_PERCENT,
	PRIMARY_PANE_SPLIT_MIN_PERCENT,
} from "$lib/constants/layout";

class LayoutState {
	/** Percent (0–100) of the split row allocated to the primary pane. */
	primaryPaneSplitPercent = $state(PRIMARY_PANE_SPLIT_DEFAULT_PERCENT);
	secondaryPaneVisible = $state(false);

	clampSplitPercent(value: number): number {
		return Math.min(
			PRIMARY_PANE_SPLIT_MAX_PERCENT,
			Math.max(PRIMARY_PANE_SPLIT_MIN_PERCENT, value),
		);
	}

	setPrimaryPaneSplitPercent(value: number): void {
		this.primaryPaneSplitPercent = this.clampSplitPercent(value);
	}

	toggleSecondaryPane(): void {
		this.secondaryPaneVisible = !this.secondaryPaneVisible;
	}
}

export const layoutState = new LayoutState();
