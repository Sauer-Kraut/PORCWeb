export function appendURLQueryParam(url: string, param_name: string, param: string | string[] | number |number[]): string {

    let base = url;
    if (!base.includes("?")) {
        base = base + "?"
    } else {
        base = base + "&"
    }

    let formated_param = "";

    if (typeof param == 'string') {
        formated_param = param;
    }

    if (typeof param == 'number') {
        formated_param = String(param);
    }

    if (Array.isArray(param) && (param.every(item => typeof item === 'number') || param.every(item => typeof item === 'string'))) {
        formated_param = param.join(',');
    }

    let res = base + param_name + '=' + formated_param;
    return res;

}