# REB CLI based time tracking

This is a fun project to learn Rust and build something useful: A simple CLI based time tracking client.

## Goals

- Learn Rust
- Build something I can use
- Fun open source project
- Community

## Product Goals

- Track time
- Time reports
- Exportable time sheets/invoices
- Always running and a keystroke (terminal) away
- Remote sync

## Feature Ideas

- Per project timers
  - Additional notes
  - Editing
  - Deleting
  - Timestamp
  - Tags
- Project metadata
  - title, name, deadlines, customer, hourly rate, budget
- Auto-start timers based on CLI commands, folders, etc.
- Auto-project organization based on computer folders
- AI augmentation: Starting timers, notes, natural language instructions, summaries, ask what you did, etc.
- Project dashboards/insights
  - Total time, deadlines,

## Design Concepts

#### Commands

**List projects|timer**
`reb ls projects|timer`

- (-p | -t short cuts)
- [x] UI prototype ✅ 2023-11-01
- [ ] Data

`reb status`

- -s short cut
- Gives status of all active timers running
- [x] UI prototype ✅ 2023-11-01
- [ ] Data

**View project|timer**
`reb project|timer id123`

- (-p | -t short cuts , or just use ID?)
- If no ID provided, it should let you pick
- [x] UI prototype
- [ ] Data

**Add project|timer**
`reb add project|timer name123`

- (-p | -t short cuts, or just use ID?)
- Create a project or timer based on the `pwd`:  `add .`
- Create and immediately start the timer: `-s`
- [ ] UI prototype
- [ ] Data

**Edit project|timer**
`reb edit project|timer id123`

- (-p | -t shortcuts, or just use ID?)
- If no ID provided, it should let you pick
- [ ] UI prototype
- [ ] Data

**Delete project|timer**
`reb delete project|timer id123`

- (-p | -t shortcuts, or just use ID?)
- If no ID provided, it should let you pick
- [ ] UI prototype
- [ ] Data

**Start/stop timer**
`reb start|stop abc123`

- (-p | -t shortcuts, or just use ID?)
- Can start a timer based on project ID (project timer) or a timer ID
- If no ID provided, it should let you pick
- `reb stop` will stop a current timer, if only one is running at the given time or the `pwd`.
- `reb start` with no ID will start a timer based on current `pwd`
- Run in the background `-b`
- [ ] UI prototype
- [ ] Data

**Export timesheet/report**
`reb report project|timer abc1234`

- (-p | -t shortcuts, or just use ID?)
- format `-f json|csv|text|pdf`
- [ ] UI prototype
- [ ] Data

**Natural language**
`reb "start the timer for ProjectName"`
`reb "list my timers"`
`reb "Stop all timers"`
`reb "Add note that I'm doing xyz"`
`reb "What did I do last hour?"`
`reb "What is the status of projectxyz"`

- [ ] UI prototype
- [ ] Data

**Config ops**
`reb config` cat it out
`code ~/.rebrc` Edit the config
`reb kill` kill the background process
`reb reboot` restart the background process

- [ ] UI prototype
- [ ] Data

## Approaches

- The less flags the better
- More natural language
- Tab is your friend (autocomplete, hinting, etc.)
- Search all the things
- Do obvious things for the user
- Portable data
- Projects, companies, or whatever are "tags" instead not their own separate objects
