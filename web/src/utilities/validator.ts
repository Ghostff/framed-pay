import {onMounted, type Ref} from "vue";

export interface ValidatableInput extends HTMLInputElement {
    validate: () => boolean
}

export class Validator {
    public isValid        = false
    public error: unknown = null

    public constructor(input: HTMLInputElement, rules: Array<string>) {
        try {
            // @ts-ignore
            for (const methodName of rules) {
                // skip all validation if not required any field is empty.
                if (methodName === 'required' && !input.required && input.value.trim() === '') {
                    break
                }
                // @ts-ignore
                this[methodName](input)
            }

            // @ts-ignore
            return (input.isValid = this.isValid = true)
        } catch (e: unknown) {
            this.error = e

            // @ts-ignore
            return (input.isValid = this.isValid = false)
        }
    }

    public static makeValidatable(input: Ref, validationError: Ref, name: string) {
        onMounted(() => {
            (input.value as ValidatableInput).validate = function () {
                validationError.value = null
                const validator       = new Validator(this, ['required', 'length', 'range', 'pattern', 'type'])

                if (!validator.isValid) {
                    validationError.value = `${name} ${validator.error}`.trim()
                }

                return validator.isValid
            }
        })
    }

    public required(input: HTMLInputElement) {
        if (input.required === undefined) {
            return
        }

        if (input.required && input.value.trim() === '') {
            throw Error('required')
        }
    }

    public type(input: HTMLInputElement) {
        if (input.type === undefined) {
            return
        }

        if (input.type === 'number' && isNaN(Number(input.value))) {
            throw Error('invalid number')
        } else if (input.type === 'email' && !/^\S+@\S+\.\S+$/.test(input.value)) {
            throw Error('invalid email')
        } else if (input.type === 'url' && this.isValidHttpUrl(input.value) === null) {
            throw Error('invalid email')
        }
    }

    public length(input: HTMLInputElement) {
        const length = input.value.trim().length
        const min    = Number(input.minLength || '-1')
        const max    = Number(input.maxLength || '-1')

        if (min === -1 && max === -1) {
            return
        }

        if (min > -1 && length < min) {
            throw Error(`value too short. min character limit is ${input.minLength}`)
        } else if (max > -1 && length > max) {
            throw Error(`value too long. max character limit is ${input.maxLength}`)
        }
    }

    public range(input: HTMLInputElement) {
        const min   = Number((input.min || '-1').trim())
        const max   = Number((input.max || '-1').trim())
        const value = Number(input.value.trim())

        if (min === -1 && max === -1) {
            return
        }

        if (min > -1 && value < min) {
            throw Error(`value cant be bellow ${min}`)
        } else if (max > -1 && value > max) {
            throw Error(`value cant be above ${max}`)
        }
    }

    public pattern(input: HTMLInputElement) {
        if ((input.pattern ?? '').length > 0 && !new RegExp(input.pattern).test(input.value)) {
            throw Error('Invalid value')
        }
    }

    private isValidHttpUrl(url: string): URL | null {
        try {
            return new URL(url)
        } catch (_) {
            return null
        }
    }
}