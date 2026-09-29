# YouVersion Platform SDK

This context covers language SDKs and add-ons that expose the YouVersion Platform API. This repository is an unofficial experiment, not an official or supported YouVersion SDK.

## Language

**YouVersion Platform API**:
The external API through which apps access YouVersion Platform resources and user-authorized data.
_Avoid_: YouVersion API, Platform service

**Platform SDK**:
A language-specific client for the YouVersion Platform API.
_Avoid_: Official SDK, API wrapper

**SDK add-on**:
A library layered over a Platform SDK to integrate it with another developer ecosystem without replacing the SDK.
_Avoid_: SDK plugin, SDK extension

**App key**:
A client credential that identifies an app and tracks its Platform API usage. It does not represent a user.
_Avoid_: Access token, user token

**Access token**:
An OAuth bearer credential that represents an authenticated user.
_Avoid_: App key, data exchange token

**Permission**:
User-granted authority for an app to access a specific kind of user data, such as highlights.
_Avoid_: OAuth scope, authorization

**Data exchange**:
The browser approval flow in which a user reviews requested permissions and returns the result to an app.
_Avoid_: OAuth flow, token exchange

**Data exchange token**:
An opaque, single-use token that carries a data exchange request into the browser approval flow and expires after five minutes.
_Avoid_: Access token, app key

**Highlight**:
A user-owned color annotation associated with a Bible version and passage.
_Avoid_: Bookmark, note

**Bible version**:
A particular published Bible text identified by a numeric ID.
_Avoid_: Bible, translation

**Passage**:
Bible text identified canonically in USFM form and represented to people with a human-readable reference.
_Avoid_: Verse, reference
