layout = "post"
title = "Unity DOTS"
created = "2023-02-25"
updated = "2023-02-25"
tags = "#unity"
markdown = """
**Since** I started my new gig at 10 Chambers, I also started using Unity DOTS. After more than a year of using DOTS in a professional capacity, I still have mixed feelings about it.

#### Burst Compiler
I want to inspect the implications of using Run(), Schedule() and ScheduleParallel(). I still don't understand the underlying implications of using them

#### Obsolete
This post is going to be a summary of the official Entities package document, kind of a cheat-sheet.

The following operations are considered structural changes:
* Creating or destroying an entity
* Adding or removing components
* Setting a shared component value

Physics
* All physics objects that are in the scene but outside the subscene will be ignored since they don't get converted to entities.

* A ```World``` organises entities into isolated groups. A World owns both an
EntityManager and a set of Sytems. In a multiplayer game, it makes sense that client and server are different Worlds.
"""