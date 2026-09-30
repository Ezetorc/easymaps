import * as v from 'valibot'

export enum ResponseStatus {
    Starting = 'Starting',
    Finished = 'Finished',
    AppError = 'AppError',
    ConnectionError = 'ConnectionError',
}

const ResponseStatusEnum = v.enum(ResponseStatus)

export const ResponseSchema = v.pipe(
    v.object({
        web_tab_id: v.optional(v.number()),
        status: ResponseStatusEnum,
    }),
    v.transform(({ web_tab_id, status }) => ({
        webTabId: web_tab_id,
        status,
    }))
)
