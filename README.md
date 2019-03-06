Support what I do on [Patreon](https://www.patreon.com/jojolepro).

# Rusty Game Features

Are you tired of games keeping their features for themselves?

Wouldn't it be great if we could share the general features that are really common, and spend more time making unique content based on those?

No one wants to spend 30 hours programming an inventory system that is (almost) the same as those currently released in closed source games.

Its time we change this. By making all the general game features, like inventory systems, user management, game chat, permissions, and many others public, the game developers will finally be able to focus on what really matters: Making their game unique and enjoyable.

This library aims to do just that. It is a repository where we will share those reusable game pieces.

## Principles

We aim to be data-oriented to facilitate the integration in whichever workflow you have.

Everything here is composed of serializable data. We are not constraining you to a specific way of handling changes in the state.

Integrating those game features is easy:
* Take the events coming from your game engine or game logic
* Use them on one or many of the features from this repository
* Notice the changes in the data
* Give feedback to your user/player

Its that simple!

Even better, since all data is meant to be serialized, and is created by composing structures together, you can easily:
* Save the game state
* Store it in a relational database
* Load it into a running game
* Inspect it
* Debug changes
* Painlessly refactor

## Features
* Extensible and fully-featured Inventory system
* Complex Loot Trees

## WIP
* Player Authentification
* User Management (kick, ban, mute, etc)
* User Permissions
* Group Based Chat Formatting and Scopes
* World ownership and protection systems


