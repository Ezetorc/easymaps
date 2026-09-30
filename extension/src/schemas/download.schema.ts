import * as v from 'valibot'

export const DownloadSchema = v.object({
    minecraftVersion: v.optional(v.string()),
    tabId: v.optional(v.number()),
    filename: v.string(),
    id: v.number(),
})

export type Download = v.InferOutput<typeof DownloadSchema>
