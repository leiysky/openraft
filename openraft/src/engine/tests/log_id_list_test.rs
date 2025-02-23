use crate::engine::LogIdList;

#[test]
fn test_log_id_list_extend_from_same_leader() -> anyhow::Result<()> {
    let mut ids = LogIdList::<u64>::default();

    // Extend one log id to an empty LogIdList: Just store it directly

    ids.extend_from_same_leader(&[log_id1(1, 2)]);
    assert_eq!(vec![log_id1(1, 2)], ids.key_log_ids());

    // Extend two log ids that are adjacent to the last stored one.
    // It should append only one log id as the new ending log id.

    ids.extend_from_same_leader(&[
        log_id1(1, 3), //
        log_id1(1, 4),
    ]);
    assert_eq!(
        vec![
            log_id1(1, 2), //
            log_id1(1, 4)
        ],
        ids.key_log_ids(),
        "same leader as the last"
    );

    // Extend 3 log id with new leader id.
    // It should just store every log id for each leader, plus one last-log-id.

    ids.extend_from_same_leader(&[
        log_id1(2, 5), //
        log_id1(2, 6),
        log_id1(2, 7),
    ]);
    assert_eq!(
        vec![
            log_id1(1, 2), //
            log_id1(2, 5),
            log_id1(2, 7)
        ],
        ids.key_log_ids(),
        "different leader as the last"
    );

    Ok(())
}

#[test]
fn test_log_id_list_extend() -> anyhow::Result<()> {
    let mut ids = LogIdList::<u64>::default();

    // Extend one log id to an empty LogIdList: Just store it directly

    ids.extend(&[log_id1(1, 2)]);
    assert_eq!(vec![log_id1(1, 2)], ids.key_log_ids());

    // Extend two log ids that are adjacent to the last stored one.
    // It should append only one log id as the new ending log id.

    ids.extend(&[
        log_id1(1, 3), //
        log_id1(1, 4),
    ]);
    assert_eq!(
        vec![
            log_id1(1, 2), //
            log_id1(1, 4)
        ],
        ids.key_log_ids(),
        "same leader as the last"
    );

    // Extend 3 log id with different leader id.
    // Last two has the same leader id.

    ids.extend(&[
        log_id1(1, 5), //
        log_id1(2, 6),
        log_id1(2, 7),
    ]);
    assert_eq!(
        vec![
            log_id1(1, 2), //
            log_id1(2, 6),
            log_id1(2, 7)
        ],
        ids.key_log_ids(),
        "last 2 have the same leaders"
    );

    // Extend 3 log id with different leader id.
    // Last two have different leader id.

    ids.extend(&[
        log_id1(2, 8), //
        log_id1(2, 9),
        log_id1(3, 10),
    ]);
    assert_eq!(
        vec![
            log_id1(1, 2), //
            log_id1(2, 6),
            log_id1(3, 10),
        ],
        ids.key_log_ids(),
        "last 2 have different leaders"
    );

    Ok(())
}

#[test]
fn test_log_id_list_append() -> anyhow::Result<()> {
    let mut ids = LogIdList::<u64>::default();

    // Append log id one by one, check the internally constructed `key_log_id` as expected.

    let cases = vec![
        (log_id1(1, 2), vec![log_id1(1, 2)]), //
        (log_id1(1, 3), vec![log_id1(1, 2), log_id1(1, 3)]),
        (log_id1(1, 4), vec![log_id1(1, 2), log_id1(1, 4)]),
        (log_id1(2, 5), vec![log_id1(1, 2), log_id1(2, 5)]),
        (log_id1(2, 7), vec![log_id1(1, 2), log_id1(2, 5), log_id1(2, 7)]),
        (log_id1(2, 9), vec![log_id1(1, 2), log_id1(2, 5), log_id1(2, 9)]),
    ];

    for (new_log_id, want) in cases {
        ids.append(new_log_id);
        assert_eq!(want, ids.key_log_ids());
    }

    Ok(())
}

#[test]
fn test_log_id_list_truncate() -> anyhow::Result<()> {
    // Sample data for test
    let make_ids = || {
        LogIdList::<u64>::new(vec![
            log_id1(2, 2), //
            log_id1(3, 3),
            log_id1(6, 6),
            log_id1(9, 9),
            log_id1(9, 11),
        ])
    };

    let cases = vec![
        (0, vec![
            //
        ]),
        (1, vec![
            //
        ]),
        (2, vec![
            //
        ]),
        (3, vec![
            log_id1(2, 2), //
        ]),
        (4, vec![
            log_id1(2, 2), //
            log_id1(3, 3),
        ]),
        (5, vec![
            log_id1(2, 2), //
            log_id1(3, 3),
            log_id1(3, 4),
        ]),
        (6, vec![
            log_id1(2, 2), //
            log_id1(3, 3),
            log_id1(3, 5),
        ]),
        (7, vec![
            log_id1(2, 2), //
            log_id1(3, 3),
            log_id1(6, 6),
        ]),
        (8, vec![
            log_id1(2, 2), //
            log_id1(3, 3),
            log_id1(6, 6),
            log_id1(6, 7),
        ]),
        (9, vec![
            log_id1(2, 2), //
            log_id1(3, 3),
            log_id1(6, 6),
            log_id1(6, 8),
        ]),
        (10, vec![
            log_id1(2, 2), //
            log_id1(3, 3),
            log_id1(6, 6),
            log_id1(9, 9),
        ]),
        (11, vec![
            log_id1(2, 2), //
            log_id1(3, 3),
            log_id1(6, 6),
            log_id1(9, 9),
            log_id1(9, 10),
        ]),
        (12, vec![
            log_id1(2, 2), //
            log_id1(3, 3),
            log_id1(6, 6),
            log_id1(9, 9),
            log_id1(9, 11),
        ]),
        (13, vec![
            log_id1(2, 2), //
            log_id1(3, 3),
            log_id1(6, 6),
            log_id1(9, 9),
            log_id1(9, 11),
        ]),
    ];

    for (at, want) in cases {
        let mut ids = make_ids();

        ids.truncate(at);
        assert_eq!(want, ids.key_log_ids(), "truncate since: [{}, +oo)", at);
    }

    Ok(())
}

#[test]
fn test_log_id_list_purge() -> anyhow::Result<()> {
    // Purge on an empty log id list:
    {
        let mut ids = LogIdList::<u64>::new(vec![]);
        ids.purge(&log_id1(2, 2));
        assert_eq!(vec![log_id1(2, 2)], ids.key_log_ids());
    }

    // Sample data for test
    let make_ids = || {
        LogIdList::<u64>::new(vec![
            log_id1(2, 2), //
            log_id1(3, 3),
            log_id1(6, 6),
            log_id1(9, 9),
            log_id1(9, 11),
        ])
    };

    let cases = vec![
        (log_id1(2, 1), vec![
            log_id1(2, 2),
            log_id1(3, 3),
            log_id1(6, 6),
            log_id1(9, 9),
            log_id1(9, 11),
        ]),
        (log_id1(2, 2), vec![
            log_id1(2, 2),
            log_id1(3, 3),
            log_id1(6, 6),
            log_id1(9, 9),
            log_id1(9, 11),
        ]),
        (log_id1(3, 3), vec![
            log_id1(3, 3),
            log_id1(6, 6),
            log_id1(9, 9),
            log_id1(9, 11),
        ]),
        (log_id1(3, 4), vec![
            log_id1(3, 4),
            log_id1(6, 6),
            log_id1(9, 9),
            log_id1(9, 11),
        ]),
        (log_id1(3, 5), vec![
            log_id1(3, 5),
            log_id1(6, 6),
            log_id1(9, 9),
            log_id1(9, 11),
        ]),
        (log_id1(6, 6), vec![log_id1(6, 6), log_id1(9, 9), log_id1(9, 11)]),
        (log_id1(6, 7), vec![log_id1(6, 7), log_id1(9, 9), log_id1(9, 11)]),
        (log_id1(6, 8), vec![log_id1(6, 8), log_id1(9, 9), log_id1(9, 11)]),
        (log_id1(9, 9), vec![log_id1(9, 9), log_id1(9, 11)]),
        (log_id1(9, 10), vec![log_id1(9, 10), log_id1(9, 11)]),
        (log_id1(9, 11), vec![log_id1(9, 11)]),
        (log_id1(9, 12), vec![log_id1(9, 12)]),
        (log_id1(10, 12), vec![log_id1(10, 12)]),
    ];

    for (upto, want) in cases {
        let mut ids = make_ids();

        ids.purge(&upto);
        assert_eq!(want, ids.key_log_ids(), "purge upto: {}", upto);
    }

    Ok(())
}

#[test]
fn test_log_id_list_get_log_id() -> anyhow::Result<()> {
    // Get log id from empty list always returns `None`.

    let ids = LogIdList::<u64>::default();

    assert!(ids.get(0).is_none());
    assert!(ids.get(1).is_none());
    assert!(ids.get(2).is_none());

    // Get log id that is a key log id or not.

    let ids = LogIdList::<u64>::new(vec![
        log_id1(1, 1),
        log_id1(1, 2),
        log_id1(3, 3),
        log_id1(5, 6),
        log_id1(7, 8),
        log_id1(7, 10),
    ]);

    assert_eq!(None, ids.get(0));
    assert_eq!(Some(log_id1(1, 1)), ids.get(1));
    assert_eq!(Some(log_id1(1, 2)), ids.get(2));
    assert_eq!(Some(log_id1(3, 3)), ids.get(3));
    assert_eq!(Some(log_id1(3, 4)), ids.get(4));
    assert_eq!(Some(log_id1(3, 5)), ids.get(5));
    assert_eq!(Some(log_id1(5, 6)), ids.get(6));
    assert_eq!(Some(log_id1(5, 7)), ids.get(7));
    assert_eq!(Some(log_id1(7, 8)), ids.get(8));
    assert_eq!(Some(log_id1(7, 9)), ids.get(9));
    assert_eq!(Some(log_id1(7, 10)), ids.get(10));
    assert_eq!(None, ids.get(11));

    Ok(())
}
use crate::testing::log_id1;
