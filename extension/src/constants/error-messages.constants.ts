import { appErrors } from '../schemas/app-response.schema'

type lol = (typeof appErrors)[number]

export const ERROR_MESSAGES: { [key in lol]: string } = {
    Unexpected:
        'An unexpected error ocurred. Check log file for more information',
    BedrockWorldNotSupported: "Bedrock worlds aren't yet supported",
    UnknownWorldVersion: "Couldn't determine map Minecraft version",
}
