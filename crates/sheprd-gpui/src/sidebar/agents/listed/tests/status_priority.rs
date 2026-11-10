use super::*;

#[test]
fn custom_priority_orders_both_generations_together() {
    let local = host(&[("p1", AgentStatus::Done), ("p2", AgentStatus::Working)]);
    let session = session(
        vec![
            agent("t1", AgentStatus::Done, "1"),
            agent("t2", AgentStatus::Working, "2"),
        ],
        &["1", "2"],
    );
    let recency = StatusRecency::default();
    let priority = [
        AgentStatus::Blocked,
        AgentStatus::Working,
        AgentStatus::Done,
        AgentStatus::Idle,
    ];
    for cores in [None, Some(core(0, &session, &recency, true))] {
        let has_core = cores.is_some();
        let rows = listed_agents(vec![(0, &local)], cores, AgentSort::Priority, &priority);
        let statuses: Vec<_> = rows.iter().map(|(_, agent)| agent.status()).collect();
        let expected = if !has_core {
            vec![AgentStatus::Working, AgentStatus::Done]
        } else {
            vec![
                AgentStatus::Working,
                AgentStatus::Working,
                AgentStatus::Done,
                AgentStatus::Done,
            ]
        };
        assert_eq!(statuses, expected);
    }
}
