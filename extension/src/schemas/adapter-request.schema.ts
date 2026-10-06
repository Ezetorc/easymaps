import * as v from 'valibot'

export const AdapterRequestSchema = v.variant('action', [
    v.object({
        action: v.literal('Start'),
        minecraftVersion: v.optional(v.string()),
    }),
])

export type AdapterRequest = v.InferOutput<typeof AdapterRequestSchema>
