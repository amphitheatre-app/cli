use crate::common::setup_mock_for;
mod common;

#[test]
fn list_actors_test() {
    let playbook_id = 1;
    let setup = setup_mock_for(
        format!("/playbooks/{}/actors", playbook_id).as_str(),
        "actors/list_actors_success",
        "GET"
    );
    let client = setup.0;

    let actors = client
        .actors()
        .list(playbook_id, None)
        .unwrap()
        .data
        .unwrap();

    assert_eq!(2, actors.len());

    let actor = actors.first().unwrap();

    assert_eq!(1, actor.id);
    assert_eq!("Default", actor.title);
    assert_eq!("First", actor.description);
    assert_eq!("2016-01-19T20:50:26Z", actor.created_at);
    assert_eq!("2016-01-19T20:50:26Z", actor.updated_at);
}

#[test]
fn get_actor_test() {
    let setup = setup_mock_for(
        "/actors/1",
        "actors/get_actor_success",
        "GET"
    );
    let client = setup.0;
    let actor_id = 1;

    let actor = client
        .actors()
        .get(actor_id)
        .unwrap()
        .data
        .unwrap();

    assert_eq!(1, actor.id);
    assert_eq!("Default", actor.title);
    assert_eq!("First", actor.description);
    assert_eq!("2016-01-19T20:50:26Z", actor.created_at);
    assert_eq!("2016-01-19T20:50:26Z", actor.updated_at);
}

#[test]
fn get_actor_logs() {
    let setup = setup_mock_for(
        "/actors/1/logs",
        "actors/get_actor_logs_success",
        "GET"
    );
    let client = setup.0;
    let actor_id = 1;

    let response = client.actors().logs(actor_id);

    assert_eq!(String::from("event stream (JSON)"), response);
}

#[test]
fn get_actor_info_test() {
    let setup = setup_mock_for(
        "/actors/1/info",
        "actors/get_actor_info_success",
        "GET"
    );
    let client = setup.0;
    let actor_id = 1;

    let json = client.actors().info(actor_id).unwrap().data.unwrap();

    assert_eq!("RdqNLMXRiRsHJhmxKurR", json["environments"]["K3S_TOKEN"]);
    assert_eq!("/var/lib/docker/volumes/f64c2f2cf81cfde89879f2a17924b31bd2f2e6a6a738f7df949bf6bd57102d25/_data", json["mounts"]["/VAR/LOG"]);
    assert_eq!("0.0.0.0:42397", json["port"]["6443/tcp"]);
}

#[test]
fn get_actor_stats_test() {
    let setup = setup_mock_for(
        "/actors/1/stats",
        "actors/get_actor_stats_success",
        "GET"
    );
    let client = setup.0;
    let actor_id = 1;

    let json = client.actors().stats(actor_id).unwrap().data.unwrap();

    assert_eq!("1.98%", json["CPU USAGE"]);
    assert_eq!("5.3MB / 43.7 MB", json["DISK READ/WRITE"]);
    assert_eq!("65.8MB", json["MEMORY USAGE"]);
    assert_eq!("5.7 kB / 3 kB", json["NETWORK I/O"]);
}