import { appErrors } from '../schemas/app-response.schema'

type ErrorKeys = (typeof appErrors)[number]

export const ERROR_MESSAGES: { [key in ErrorKeys]: string } = {
    Unexpected: 'Unexpected error. Check logs for more information',
    BedrockWorldNotSupported: "Bedrock worlds aren't yet supported",
    UnknownWorldVersion: "Couldn't determine map Minecraft version",
    WorldNotFound: 'World not found',
}
