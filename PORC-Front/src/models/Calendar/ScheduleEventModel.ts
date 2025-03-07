export interface ScheduleEvent {
    startDate: Date;
    endDate: Date;
    repetition: Repetition;
    repetition_config: DailyRepetitionConfig;
}

export enum Repetition {
  Once = 'Once',
  Daily = 'Daily',
  Weekly = 'Weekly',
  Monthly = 'Monthly',
  Yearly = 'Yearly',
}

export interface DailyRepetitionConfig {
  monday: boolean;
  tuesday: boolean;
  wednesday: boolean;
  thursday: boolean;
  friday: boolean;
  saturday: boolean;
  sunday: boolean;
}

