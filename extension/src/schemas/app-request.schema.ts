import * as v from 'valibot'

export const AppRequestSchema = v.variant('action', [
    v.object({
        action: v.literal('Start'),
        minecraft_version: v.optional(v.string()),
        download_path: v.string(),
        requester_id: v.number(),
    }),
])

export type AppRequest = v.InferOutput<typeof AppRequestSchema>
