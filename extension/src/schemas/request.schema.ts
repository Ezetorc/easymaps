import * as v from 'valibot'

export enum RequestAction {
    Start = 'Start',
}

const RequestActionEnum = v.enum(RequestAction)

export const RequestSchema = v.object({
    minecraftVersion: v.pipe(v.string(), v.nonEmpty()),
    action: RequestActionEnum,
})
