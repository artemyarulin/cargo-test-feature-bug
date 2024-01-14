# cargo-test-feature-dependency bug

Different behavior in features resolution during testing between `cargo test` on workspace level and `cargo test` in every workspace member.

Workspace contains of two members - `a` and `b`. 

Member `a` contains an optional feature `foo` and when tested with this feature on - panics. By default `foo` is not enabled so `cd a && cargo test` succeeds as test is skipped.

Member `b` references `a` with feature `foo` enabled. Although `cd b && cargo test` succeed as when tests are run - tests of dependencies are skipped.

But if `cargo test` is run on workspace level then tests fails and `a` test runs which is not expected.




