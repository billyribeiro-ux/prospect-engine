import type { AuditDimensionId } from "./audit";

export interface CrawlSummary {
	readonly pagesVisited: number;
	readonly pagesSucceeded: number;
	readonly pagesFailed: number;
	readonly totalBytes: number;
}

export interface DimensionScores {
	readonly performance: number;
	readonly seo: number;
	readonly content: number;
	readonly design: number;
	readonly accessibility: number;
	readonly security: number;
	readonly tech_debt: number;
}

export type ServerEvent =
	| { type: "crawl:started"; jobId: string; url: string; totalPages: number }
	| {
			type: "crawl:page_complete";
			jobId: string;
			url: string;
			statusCode: number;
			loadTimeMs: number;
	  }
	| { type: "crawl:page_error"; jobId: string; url: string; error: string }
	| { type: "crawl:complete"; jobId: string; summary: CrawlSummary }
	| {
			type: "audit:progress";
			jobId: string;
			dimension: AuditDimensionId;
			progress: number;
	  }
	| {
			type: "audit:complete";
			jobId: string;
			scores: DimensionScores;
			composite: number;
	  }
	| {
			type: "queue:status";
			pending: number;
			active: number;
			completed: number;
			failed: number;
	  }
	| { type: "error"; code: string; message: string };

export type ClientEvent =
	| { type: "crawl:cancel"; jobId: string }
	| { type: "crawl:pause_all" }
	| { type: "crawl:resume_all" }
	| { type: "subscribe:job"; jobId: string }
	| { type: "unsubscribe:job"; jobId: string };
