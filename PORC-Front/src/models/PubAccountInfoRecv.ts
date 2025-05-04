import config from '@/config';
import type { MatchEvent, MatchStatus } from './Calendar/MatchEventModel';
import type { DailyRepetitionConfig, Repetition, ScheduleEvent } from './Calendar/ScheduleEventModel';
import type { Schedule } from './Calendar/ScheduleModel';
import type { PubAccountInfo } from './PubAccountInfo';
import { showErrorModal } from '@/services/ErrorModalService';

export interface PubAccountInfoRecv {
    id: number;
    username: string;
    avatar: string | null;
    schedule: ScheduleRecv | null;
}

export interface ScheduleRecv {
    availabilities: ScheduleEventRecv[];
    matches: number[];
    note: string;
}

export interface ScheduleEventRecv {
    start_timestamp: number;
    end_timestamp: number;
    repetition: Repetition;
    repetition_config: DailyRepetitionConfig | null;
}

export interface MatchEventRecv {
    id: number | null;
    start_timestamp: number;
    challenger_id: number;
    opponent_id: number;
    event_id: number | null;
    status: MatchStatus;
    season: String;
}

export async function convertToPubAccountInfo(recv: PubAccountInfoRecv): Promise<PubAccountInfo> {
    //console.log("Received data:", recv); // Add this line to log the received data
    if (recv.schedule) {
        //console.log("Schedule data:", recv.schedule); // Add this line to log the schedule data
        recv.schedule.availabilities.forEach((event, index) => {
            //console.log(`Availability ${index}:`, event); // Add this line to log each availability event
        });
    }

    let matches = recv.schedule ? await getMatchEvents(recv.schedule.matches) : [];
    const dummySchedule = ({
        availabilities: [] as ScheduleEvent[],
        matches: [] as MatchEvent[],
        note: "",
    } as Schedule)
    return {
        id: recv.id,
        username: recv.username,
        avatar: recv.avatar,
        schedule: recv.schedule
            ? ({
                  availabilities: recv.schedule.availabilities.map((event) => {
                      //console.log("Event timestamps:", event.start_timestamp, event.end_timestamp); // Add this line to log event timestamps
                      return {
                          startDate: new Date(event.start_timestamp * 1000),
                          endDate: new Date(event.end_timestamp * 1000),
                          repetition: event.repetition,
                          repetition_config: event.repetition_config
                              ? event.repetition_config
                              : {
                                    monday: false,
                                    tuesday: false,
                                    wednesday: false,
                                    thursday: false,
                                    friday: false,
                                    saturday: false,
                                    sunday: false,
                                },
                      } as ScheduleEvent;
                  }),
                  matches: matches.map((match) => {
                      //console.log("Match timestamp:", match.start_timestamp); // Add this line to log match timestamp
                      return {
                          startDate: new Date(match.start_timestamp * 1000),
                          endDate: new Date(match.start_timestamp * 1000 + 3600000),
                          initiatorId: match.challenger_id,
                          opponentId: match.opponent_id,
                          status: match.status,
                      };
                  }),
                  note: recv.schedule.note,
              } as Schedule)
            : dummySchedule,
    };
}

export function convertToPubAccountInfoRecv(info: PubAccountInfo): PubAccountInfoRecv {
    //console.log("Converting to PubAccountInfoRecv:", info); // Add this line to log the info object
    return {
        id: info.id,
        username: info.username,
        avatar: info.avatar,
        schedule: info.schedule
            ? {
                  availabilities: info.schedule.availabilities.map(
                      (event) =>
                          ({
                              start_timestamp: Math.floor(event.startDate.getTime() / 1000),
                              end_timestamp: Math.floor(event.endDate.getTime() / 1000),
                              repetition: event.repetition,
                              repetition_config: event.repetition_config,
                          }) as ScheduleEventRecv,
                  ),
                  matches: [], // Keep the match array empty
                  note: info.schedule.note,
              }
            : null,
    };
}

export function convertToMatchEventRecv(event: MatchEvent, season: string): MatchEventRecv {
    return {
        start_timestamp: Math.floor(event.startDate.getTime() / 1000),
        challenger_id: event.initiatorId,
        opponent_id: event.opponentId,
        status: event.status,
        id: null,
        event_id: null,
        season: season
    };
}

async function getMatchEvents(match_ids: number[]): Promise<MatchEventRecv[]> {
    //console.log('Trying to get match events for the following match ids: ', match_ids);

    try {
        const response = await fetch(`${config.getBackendUrl()}/api/matches/get`, {
            method: 'PUT',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify({
                title: 'Match event info Request',
                match_events: match_ids,
            }),
        });

        if (!response.ok) {
            throw new Error('Network response was not ok');
        }

        let data = await response.json();
        //console.log('Success:', data);

        if (data.error != null) {
            let errorMessage = data.error; // TODO: Add error handling
            showErrorModal(errorMessage);
            //console.log('Error message:', errorMessage);
            return [];
        } else {
            let matchEvents: MatchEventRecv[] = data.data;
            //console.log('Received response for match events: ', matchEvents);
            return matchEvents;
        }
    } catch (error) {
        console.error('Error:', error); // TODO: Add error handling
        return [];
    }
}
