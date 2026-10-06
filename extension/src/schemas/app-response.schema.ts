import * as v from 'valibot'

export const appErrors = [
    'Unexpected',
    'BedrockWorldNotSupported',
    'UnknownWorldVersion',
] as const

export const AppResponseSchema = v.variant('status', [
    v.object({
        status: v.literal('Importing'),
        requester_id: v.number(),
    }),

    v.object({
        status: v.literal('Installing'),
        requester_id: v.number(),
    }),

    v.object({
        status: v.literal('Launching'),
        requester_id: v.number(),
    }),

    v.object({
        status: v.literal('AppError'),
        requester_id: v.number(),
        error: v.picklist(appErrors),
    }),

    v.object({
        status: v.literal('ConnectionError'),
    }),
])

export type AppResponse = v.InferOutput<typeof AppResponseSchema>
