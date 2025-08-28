# abit-rs

This repository provides a focused parser for admissions data exported from the Ukrainian EDBO system. Its purpose is simple and practical: to reliably read and normalize raw EDBO exports, extract applicant records, preference lists, and contest metadata, and prepare that information for further analysis or processing.

The project contains parsing code only - there is no placement, allocation, or quota algorithm included here because I did not finish that part before publishing the results. The implementation favors clarity, deterministic behavior, and minimal dependencies to make the parser easy to inspect, test, and integrate into other tools.
