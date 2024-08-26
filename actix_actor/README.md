# Actor

The actor model in computer science is a mathematical model of concurrent computation that treats an actor as the basic building block of concurrent computation. In response to a message it receives, an actor can: make local decisions, create more actors, send more messages, and determine how to respond to the next message received. Actors may modify their own private state, but can only affect each other indirectly through messaging (removing the need for lock-based synchronization).

## References

- [Actor model](https://en.wikipedia.org/wiki/Actor_model)
- [Actix Actor](https://actix.rs/docs/actix/actor)
