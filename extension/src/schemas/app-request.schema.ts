import * as v from 'valibot'

export const AppRequestSchema = v.variant('action', [
    v.object({
        action: v.literal('StartMap'),
        minecraft_version: v.optional(v.string()),
        minecraft_username: v.optional(v.string()),
        minecraft_world_name: v.string(),
        download_path: v.string(),
        requester_id: v.number(),
    }),
    v.object({
        action: v.literal('CheckMap'),
        world_name: v.string(),
        requester_id: v.number(),
    }),
    v.object({
        action: v.literal('ContinueMap'),
        world_name: v.string(),
        requester_id: v.number(),
        minecraft_version: v.optional(v.string()),
        minecraft_username: v.optional(v.string()),
    }),
    v.object({
        action: v.literal('DeleteMap'),
        world_name: v.string(),
        requester_id: v.number(),
    }),
])

export type AppRequest = v.InferOutput<typeof AppRequestSchema>
