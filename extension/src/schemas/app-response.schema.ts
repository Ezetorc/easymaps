import * as v from 'valibot'

export const appErrors = [
    'Unexpected',
    'BedrockWorldNotSupported',
    'UnknownWorldVersion',
    'WorldNotFound',
] as const

export const AppResponseSchema = v.variant('status', [
    v.object({
        status: v.literal('ImportingMap'),
        requester_id: v.number(),
    }),

    v.object({
        status: v.literal('InstallingVersion'),
        requester_id: v.number(),
    }),

    v.object({
        status: v.literal('LaunchingMinecraft'),
        requester_id: v.number(),
    }),

    v.object({
        status: v.literal('CheckingMap'),
        requester_id: v.number(),
    }),

    v.object({
        status: v.literal('MapCheckResult'),
        world_exists: v.boolean(),
        requester_id: v.number(),
    }),

    v.object({
        status: v.literal('DeletingMap'),
        requester_id: v.number(),
    }),

    v.object({
        status: v.literal('MapDeleteSuccess'),
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
