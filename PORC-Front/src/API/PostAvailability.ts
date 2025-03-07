import type { ScheduleEvent } from "../models/Calendar/ScheduleEventModel";
import type { Schedule } from "../models/Calendar/ScheduleModel";
import { getClientId } from "./clientIdentification";
import { getLoggedIn } from "./GetLoggedIn";
import { postUserInfo } from "./PostAccountInfo";

async function AddAvailability(avail: ScheduleEvent): Promise<void | string> {
    console.log('Trying to add availability');

    let account = await getLoggedIn();
    if (typeof account === 'string') {
        return account;
    }

    if (account.schedule) {
        account.schedule.availabilities.push(avail);
    } else {
        account.schedule = {
            availabilities: [avail],
            matches: [],
            notes: '',
        } as Schedule
    }

    let err = await postUserInfo(account);
    return err;
}

async function RemoveAvailability(avail: ScheduleEvent): Promise<void | string> {
    console.log('Trying to remove availability');

    let account = await getLoggedIn();
    if (typeof account === 'string') {
        return account;
    }

    if (account.schedule) {
        let newAvailabilities = account.schedule.availabilities.filter(availability => availability !== avail);
        if (newAvailabilities === account.schedule.availabilities) {
            return 'Availability not found';
        }
        account.schedule.availabilities = newAvailabilities;
    } else {
        account.schedule = {
            availabilities: [],
            matches: [],
            notes: '',
        } as Schedule
    }

    let err = await postUserInfo(account);
    return err;
}