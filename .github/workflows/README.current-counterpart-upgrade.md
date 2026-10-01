# Current production upgrade lane

This repository now runs deterministic local upgrade-compatibility tests before inspecting the exact current production counterpart tips. The production check remains fail-closed when private counterpart repositories are not visible and expects `TEST_FLEET_GITHUB_TOKEN` (or an equivalent cross-org read credential) for private source access.
