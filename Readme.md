# Antitelephone - A Causality Violating Bot for Discord

Antitelephone is a discord bot for scheduling one-off messages. Use the command "anti.msg 5h30m (your message)" and antitelephone will consume your message (deleting it) and will repost it in an embed in five hours and thirty minutes. Use "anti.?" to get the full help manual. Note some features in the manual are unimplemented.

### To-Do list, in order of difficulty:

- [ ] print out time difference in a nice way (5 mins from now/3 hours ago/etc)
- [ ] allow offset parameter to be an actual date
- [x] implement list command
- [ ] implement delete command
- [ ] per-channel-id limits on number of items in queue
- [ ] see aboot using serenity's built-in help functionality
- [ ] throttle backups
- [ ] ask channels if they would like to receive overdue messages if the bot crashed (need message limits first)
- [x] ~~throttled~~ backups of the queue ~~(i.e. a few seconds after someone sends a message, the timer of which is cancelled whenever a new message comes in, unless it is overdue, in which case it happens immediately....?)~~ (URGENT)
- [x] loading queue from disk, and ~~asking channels that have overdue messages if they would like to receive them~~ (URGENT)
- [ ] implement user-defined input/output channels, so the input channel can be muted so nobody will see notifications of the consumed messages
