import { accountsStore } from "@/logic/storage/st_accounts";
import { type Availability } from "@/models/pub_account_info/schedule/availability/Availability";
import type { AvailabilityException } from "@/models/pub_account_info/schedule/availability_exception/AvailabilityException";
import type { MatchEvent } from "@/models/pub_account_info/schedule/match_event/MatchEvent";



export async function submitMatchRequest(data: MatchEvent, emit: (event: 'reload') => void, closeCalendarPopover: () => void): Promise<void> {
    closeCalendarPopover();
    const compStore = accountsStore();
    await compStore.create_match_event_local(data);
    await compStore.post_match_event(data);
    emit('reload');
}

export async function updateAvailabilities(updated: Availability[], original: Availability[], emit: (event: 'reload') => void): Promise<void> {
    const compStore = accountsStore();
    await compStore.self_edit_availabilities_local(updated, original);
    await compStore.post_account();
    emit('reload');
}

export async function updateExceptions(updated: AvailabilityException[], original: AvailabilityException[], emit: (event: 'reload') => void): Promise<void> {
    const compStore = accountsStore();
    await compStore.self_edit_exceptions_local(updated, original);
    await compStore.post_account();
    emit('reload');
}

export async function deleteAvailabilities(availability: Availability[], emit: (event: 'reload') => void, closeCalendarPopover: () => void): Promise<void> {
    const compStore = accountsStore();
    closeCalendarPopover();
    await compStore.self_edit_availabilities_local([], availability);
    await compStore.post_account();
    emit('reload');
}
