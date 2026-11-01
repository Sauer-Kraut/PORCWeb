export const TIMEZONES = [
    "UTC",
    "America/Los_Angeles",
    "America/Denver",
    "America/Chicago",
    "America/New_York",
    "America/Sao_Paulo",
    "Europe/London",
    "Europe/Paris",
    "Europe/Zurich",
    "Europe/Helsinki",
    "Europe/Moscow",
    "Africa/Cairo",
    "Africa/Johannesburg",
    "Asia/Dubai",
    "Asia/Kolkata",
    "Asia/Bangkok",
    "Asia/Singapore",
    "Asia/Shanghai",
    "Asia/Seoul",
    "Asia/Tokyo",
    "Australia/Perth",
    "Australia/Sydney",
    "Pacific/Auckland",
];

export function timezoneLabel(timeZone: string): string {
    const parts = new Intl.DateTimeFormat("en", {
        timeZone,
        timeZoneName: "longOffset",
    }).formatToParts(new Date());

    const offset =
        parts.find(p => p.type === "timeZoneName")?.value ?? "";

    return `(${offset}) ${timeZone}`;
}

export const TIMEZONE_OPTIONS = [
    { value: "UTC", label: "(UTC+00:00) UTC" },

    { value: "America/Los_Angeles", label: "(UTC-07:00) Pacific Time" },
    { value: "America/Denver",      label: "(UTC-06:00) Mountain Time" },
    { value: "America/Chicago",     label: "(UTC-05:00) Central Time" },
    { value: "America/New_York",    label: "(UTC-04:00) Eastern Time" },

    { value: "America/Sao_Paulo",   label: "(UTC-03:00) São Paulo" },

    { value: "Europe/London",       label: "(UTC+01:00) London" },
    { value: "Europe/Paris",        label: "(UTC+02:00) Paris" },
    { value: "Europe/Zurich",       label: "(UTC+02:00) Zurich" },
    { value: "Europe/Helsinki",     label: "(UTC+03:00) Helsinki" },
    { value: "Europe/Moscow",       label: "(UTC+03:00) Moscow" },

    { value: "Africa/Cairo",        label: "(UTC+03:00) Cairo" },
    { value: "Africa/Johannesburg", label: "(UTC+02:00) Johannesburg" },

    { value: "Asia/Dubai",          label: "(UTC+04:00) Dubai" },
    { value: "Asia/Kolkata",        label: "(UTC+05:30) India" },
    { value: "Asia/Bangkok",        label: "(UTC+07:00) Bangkok" },
    { value: "Asia/Singapore",      label: "(UTC+08:00) Singapore" },
    { value: "Asia/Shanghai",       label: "(UTC+08:00) Shanghai" },
    { value: "Asia/Seoul",          label: "(UTC+09:00) Seoul" },
    { value: "Asia/Tokyo",          label: "(UTC+09:00) Tokyo" },

    { value: "Australia/Perth",     label: "(UTC+08:00) Perth" },
    { value: "Australia/Sydney",    label: "(UTC+10:00) Sydney" },
    { value: "Pacific/Auckland",    label: "(UTC+12:00) Auckland" },
];