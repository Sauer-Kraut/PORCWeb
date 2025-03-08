import type { DailyRepetitionConfig, ScheduleEvent } from '../models/Calendar/ScheduleEventModel';
import type { Schedule } from '../models/Calendar/ScheduleModel';
import { getClientId } from './clientIdentification';
import { getLoggedIn } from './GetLoggedIn';
import { postUserInfo } from './PostAccountInfo';

export async function EditAvailability(addAvail: ScheduleEvent[], remAvail: ScheduleEvent[]): Promise<void | string> {
    //console.log('Trying to add / remove availability');
    //console.log("availabilities to remove: ", remAvail);

    let account = await getLoggedIn();
    if (typeof account === 'string') {
        return account;
    }

    if (account.schedule) {
        for (let avail of remAvail) {
            //console.log("avail to delete: ", avail);
            
            const delAvail = {
                startDate: avail.startDate,
                endDate: avail.endDate,
                repetition: avail.repetition,
                repetition_config: {
                    monday: avail.repetition_config.monday,
                    tuesday: avail.repetition_config.tuesday,
                    wednesday: avail.repetition_config.wednesday,
                    thursday: avail.repetition_config.thursday,
                    friday: avail.repetition_config.friday,
                    saturday: avail.repetition_config.saturday,
                    sunday: avail.repetition_config.sunday
                } as DailyRepetitionConfig
            } as ScheduleEvent
            //console.log("avail to delete: ", delAvail);

            function areScheduleEventsEqual(event1: ScheduleEvent, event2: ScheduleEvent): boolean {
                if (event1.startDate.getTime() !== event2.startDate.getTime()) {
                    console.log(`startDate check failed: ${event1.startDate} !== ${event2.startDate}`);
                    return false;
                }
                if (event1.endDate.getTime() !== event2.endDate.getTime()) {
                    console.log(`endDate check failed: ${event1.endDate} !== ${event2.endDate}`);
                    return false;
                }
                if (event1.repetition !== event2.repetition) {
                    console.log(`repetition check failed: ${event1.repetition} !== ${event2.repetition}`);
                    return false;
                }
                if (event1.repetition_config.monday !== event2.repetition_config.monday) {
                    console.log(`monday check failed: ${event1.repetition_config.monday} !== ${event2.repetition_config.monday}`);
                    return false;
                }
                if (event1.repetition_config.tuesday !== event2.repetition_config.tuesday) {
                    console.log(`tuesday check failed: ${event1.repetition_config.tuesday} !== ${event2.repetition_config.tuesday}`);
                    return false;
                }
                if (event1.repetition_config.wednesday !== event2.repetition_config.wednesday) {
                    console.log(`wednesday check failed: ${event1.repetition_config.wednesday} !== ${event2.repetition_config.wednesday}`);
                    return false;
                }
                if (event1.repetition_config.thursday !== event2.repetition_config.thursday) {
                    console.log(`thursday check failed: ${event1.repetition_config.thursday} !== ${event2.repetition_config.thursday}`);
                    return false;
                }
                if (event1.repetition_config.friday !== event2.repetition_config.friday) {
                    console.log(`friday check failed: ${event1.repetition_config.friday} !== ${event2.repetition_config.friday}`);
                    return false;
                }
                if (event1.repetition_config.saturday !== event2.repetition_config.saturday) {
                    console.log(`saturday check failed: ${event1.repetition_config.saturday} !== ${event2.repetition_config.saturday}`);
                    return false;
                }
                if (event1.repetition_config.sunday !== event2.repetition_config.sunday) {
                    console.log(`sunday check failed: ${event1.repetition_config.sunday} !== ${event2.repetition_config.sunday}`);
                    return false;
                }
                return true;
            }

            let newAvailabilities = account.schedule.availabilities.filter((availability) => !areScheduleEventsEqual(availability, delAvail));
            if (newAvailabilities == account.schedule.availabilities) {
                return 'Availabilities to remove not found';
            } else {
                //console.log("narrowed avails: ", newAvailabilities);
            }
            account.schedule.availabilities = newAvailabilities;
            //console.log("account schedule: ", account.schedule.availabilities)
        }
        account.schedule.availabilities.push(...addAvail);
    } else {
        account.schedule = {
            availabilities: addAvail,
            matches: [],
            notes: '',
        } as Schedule;
    }

    //console.log("new Availabilities: ", account.schedule.availabilities)

    let err = await postUserInfo(account);
    return err;
}
