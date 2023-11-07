import axios, {AxiosResponse} from 'axios';

export interface RecaptchaRequestInterface {
    method?: string;
    uri: string;
    data?: object;
    params?: object;
}

export function recaptchaRequest(config: RecaptchaRequestInterface): Promise<AxiosResponse> {
    return new Promise((resolve, reject) => {
        // @ts-ignore
        grecaptcha.enterprise.ready(() => {
            // @ts-ignore
            grecaptcha.enterprise.execute(import.meta.env.VITE_GOOGLE_RECAPTCHA_KEY, {action: 'FORM_SUBMIT'})
                .then((recaptchaToken: string) => {
                    if (config.data) {
                        // @ts-ignore
                        config.data.recaptcha_token = recaptchaToken;
                    }

                    axios.request({
                        method: config.method || 'POST',
                        url: config.uri,
                        data: config.data,
                        params: config.params,
                    }).then(r => resolve(r)).catch(e => reject(e));
                })
                .catch((error: any) => {
                    reject(error);
                });
        });
    })

}