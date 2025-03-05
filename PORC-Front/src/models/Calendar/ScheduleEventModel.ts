export interface ScheduleEvent {
    startDate: Date;
    endDate: Date;
    repetition: Repetition;
}

export type Repetition =
  | { type: "Once" }
  | { type: "Daily"; data: DailyRepetitionConfig }
  | { type: "Weekly" }
  | { type: "Monthly" }
  | { type: "Yearly" };

export interface DailyRepetitionConfig {
  monday: boolean;
  tuesday: boolean;
  wednesday: boolean;
  thursday: boolean;
  friday: boolean;
  saturday: boolean;
  sunday: boolean;
}

