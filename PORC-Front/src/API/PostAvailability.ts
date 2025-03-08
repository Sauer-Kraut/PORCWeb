import type { ScheduleEvent } from "../models/Calendar/ScheduleEventModel";
import type { Schedule } from "../models/Calendar/ScheduleModel";
import { getClientId } from "./clientIdentification";
import { getLoggedIn } from "./GetLoggedIn";
import { postUserInfo } from "./PostAccountInfo";

export async function EditAvailability(addAvail: ScheduleEvent[], remAvail: ScheduleEvent[]): Promise<void | string> {
    console.log('Trying to add availability');
    console.log("availabilities to remove: ", remAvail);

    let account = await getLoggedIn();
    if (typeof account === 'string') {
        return account;
    }

    if (account.schedule) {
        for (let avail of remAvail) {
            let newAvailabilities = account.schedule.availabilities.filter(availability => availability != avail);
            if (newAvailabilities === account.schedule.availabilities) {
                return 'Availabilities to remove not found';
            }
            account.schedule.availabilities = newAvailabilities;
            console.log("account schedule: ", account.schedule.availabilities)
        }
        
        account.schedule.availabilities.push(...addAvail);
    } else {
        account.schedule = {
            availabilities: addAvail,
            matches: [],
            notes: '',
        } as Schedule
    }

    console.log("new Availabilities: ", account.schedule.availabilities)

    let err = await postUserInfo(account);
    return err;
}