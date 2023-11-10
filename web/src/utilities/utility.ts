import {reactive} from "vue";
export function resettableReactive<T extends object>(obj: T) {
    const react = reactive({
        ...obj,
        reset() {
            Object.assign(react, obj)
        }
    });

    return react;
}

export function formatPhoneNumber(e164Number: string): string {
    e164Number = e164Number.replace(/\D/g, '');
    const match = e164Number.match(/^(\d{1,3})(\d{0,3})?(\d+)?/);

    console.log(match)
    if (match) {
        let formattedNumber = `+${match[1]}`;
        if (match[2]) {
            formattedNumber += ` (${match[2]})`;
        }
        if (match[3]) {
            formattedNumber += ` ${match[3]}`;
        }
        return formattedNumber;
    }

    return e164Number;
}


