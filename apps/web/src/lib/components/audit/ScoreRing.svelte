<script lang="ts">
interface Props {
	score?: number;
	max?: number;
	size?: number;
}

let { score = 0, max = 100, size = 64 }: Props = $props();

const radius = $derived((size - 8) / 2);
const circumference = $derived(2 * Math.PI * radius);
const offset = $derived(circumference - (score / max) * circumference);
</script>

<div class="pe-score-ring" style={`width:${size}px;height:${size}px`} aria-label={`Score ${score} of ${max}`}>
	<svg width={size} height={size} viewBox={`0 0 ${size} ${size}`} role="img">
		<title>Score {score}</title>
		<circle
			class="pe-score-ring__track"
			cx={size / 2}
			cy={size / 2}
			r={radius}
			fill="none"
			stroke="var(--pe-border-subtle)"
			stroke-width="4"
		/>
		<circle
			class="pe-score-ring__value"
			cx={size / 2}
			cy={size / 2}
			r={radius}
			fill="none"
			stroke="var(--pe-interactive-primary)"
			stroke-width="4"
			stroke-dasharray={circumference}
			stroke-dashoffset={offset}
			transform={`rotate(-90 ${size / 2} ${size / 2})`}
		/>
	</svg>
	<span class="pe-score-ring__label">{Math.round(score)}</span>
</div>

<style>
	@layer components {
		.pe-score-ring {
			position: relative;
			display: grid;
			place-items: center;
			font-family: var(--pe-font-family);
		}

		.pe-score-ring__value {
			transition: stroke-dashoffset 0.6s var(--pe-easing-default, ease);
		}

		.pe-score-ring__label {
			position: absolute;
			font-size: var(--pe-font-size-md);
			font-weight: 700;
			color: var(--pe-text-primary);
		}
	}
</style>
