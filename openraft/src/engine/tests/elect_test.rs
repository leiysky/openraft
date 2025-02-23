use std::collections::BTreeSet;
use std::sync::Arc;

use maplit::btreeset;
use pretty_assertions::assert_eq;
use tokio::time::Instant;

use crate::core::ServerState;
use crate::engine::testing::UTConfig;
use crate::engine::Command;
use crate::engine::Engine;
use crate::engine::LogIdList;
use crate::entry::RaftEntry;
use crate::raft::VoteRequest;
use crate::testing::log_id;
use crate::testing::log_id1;
use crate::utime::UTime;
use crate::CommittedLeaderId;
use crate::EffectiveMembership;
use crate::Entry;
use crate::LogId;
use crate::Membership;
use crate::Vote;

fn m1() -> Membership<u64, ()> {
    Membership::new(vec![btreeset! {1}], None)
}

fn m12() -> Membership<u64, ()> {
    Membership::new(vec![btreeset! {1,2}], None)
}

fn eng() -> Engine<UTConfig> {
    let mut eng = Engine::default();
    eng.state.log_ids = LogIdList::new([LogId::new(CommittedLeaderId::new(0, 0), 0)]);
    eng.state.enable_validate = false; // Disable validation for incomplete state
    eng
}

#[test]
fn test_elect() -> anyhow::Result<()> {
    tracing::info!("--- single node: become leader at once");
    {
        let mut eng = eng();
        eng.config.id = 1;
        eng.state
            .membership_state
            .set_effective(Arc::new(EffectiveMembership::new(Some(log_id1(0, 1)), m1())));

        eng.elect();

        assert_eq!(Vote::new_committed(1, 1), *eng.state.vote_ref());
        assert_eq!(
            Some(btreeset! {1},),
            eng.internal_server_state.leading().map(|x| x.voting().granters().collect::<BTreeSet<_>>())
        );

        assert_eq!(ServerState::Leader, eng.state.server_state);

        assert_eq!(
            vec![
                Command::SaveVote { vote: Vote::new(1, 1) },
                Command::SaveVote {
                    vote: Vote::new_committed(1, 1)
                },
                Command::BecomeLeader,
                Command::RebuildReplicationStreams { targets: vec![] },
                Command::AppendEntry {
                    entry: Entry::<UTConfig>::new_blank(log_id(1, 1, 1))
                },
                Command::ReplicateCommitted {
                    committed: Some(LogId {
                        leader_id: CommittedLeaderId::new(1, 1),
                        index: 1,
                    },),
                },
                Command::Apply {
                    seq: 1,
                    already_committed: None,
                    upto: LogId {
                        leader_id: CommittedLeaderId::new(1, 1),
                        index: 1,
                    },
                },
            ],
            eng.output.take_commands()
        );
    }

    tracing::info!("--- single node: electing again will override previous state");
    {
        let mut eng = eng();
        eng.config.id = 1;
        eng.state
            .membership_state
            .set_effective(Arc::new(EffectiveMembership::new(Some(log_id1(0, 1)), m1())));

        // Build in-progress election state
        eng.state.vote = UTime::new(Instant::now(), Vote::new_committed(1, 2));
        eng.vote_handler().become_leading();
        eng.internal_server_state.leading_mut().map(|l| l.voting_mut().grant_by(&1));

        eng.elect();

        assert_eq!(Vote::new_committed(2, 1), *eng.state.vote_ref());
        assert_eq!(
            Some(btreeset! {1},),
            eng.internal_server_state.leading().map(|x| x.voting().granters().collect::<BTreeSet<_>>())
        );

        assert_eq!(ServerState::Leader, eng.state.server_state);

        assert_eq!(
            vec![
                Command::SaveVote { vote: Vote::new(2, 1) },
                Command::SaveVote {
                    vote: Vote::new_committed(2, 1)
                },
                Command::BecomeLeader,
                Command::RebuildReplicationStreams { targets: vec![] },
                Command::AppendEntry {
                    entry: Entry::<UTConfig>::new_blank(log_id(2, 1, 1))
                },
                Command::ReplicateCommitted {
                    committed: Some(LogId {
                        leader_id: CommittedLeaderId::new(2, 1),
                        index: 1,
                    },),
                },
                Command::Apply {
                    seq: 1,
                    already_committed: None,
                    upto: LogId {
                        leader_id: CommittedLeaderId::new(2, 1),
                        index: 1,
                    },
                },
            ],
            eng.output.take_commands()
        );
    }

    tracing::info!("--- multi nodes: enter candidate state");
    {
        let mut eng = eng();
        eng.config.id = 1;
        eng.state
            .membership_state
            .set_effective(Arc::new(EffectiveMembership::new(Some(log_id1(0, 1)), m12())));
        eng.state.log_ids = LogIdList::new(vec![log_id1(1, 1)]);

        eng.elect();

        assert_eq!(Vote::new(1, 1), *eng.state.vote_ref());
        assert_eq!(
            Some(btreeset! {1},),
            eng.internal_server_state.leading().map(|x| x.voting().granters().collect::<BTreeSet<_>>())
        );

        assert_eq!(ServerState::Candidate, eng.state.server_state);

        assert_eq!(
            vec![Command::SaveVote { vote: Vote::new(1, 1) }, Command::SendVote {
                vote_req: VoteRequest::new(Vote::new(1, 1), Some(log_id1(1, 1)))
            },],
            eng.output.take_commands()
        );
    }
    Ok(())
}
