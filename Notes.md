
# Developer Notes

## Create Snapshot Release

First create or move the tag `snapshot` and then push it to remote.
Both commands need `--force` as the tag `snapshot` already exists.

    git tag --force snapshot
    git push origin snapshot --force


## Create a Release

1. Update version number in `Cargo.toml`
2. Update `Changelog.md`:
   * Set _release version_ and _date_
   * Check and update _release compare link_
3. Run local check and build: `make check rpm deb tar zip` and verify results
4. Commit changes
5. Create tag. Comment like "Release 0.2.0".
6. Push code: `git push`
7. Push tag only: `git push origin v0.2.0`
8. Wait till GitHub workflow `release` is done.
9. Check created release.

