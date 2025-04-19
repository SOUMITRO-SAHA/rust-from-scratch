use rust::chapter16::quiz::*;

#[test]
fn test_spawn_thread() {
    spawn_thread();
}

#[test]
fn test_channel_send_recv() {
    assert_eq!(channel_send_recv(77), 77);
}

#[test]
fn test_arc_mutex_increment() {
    assert_eq!(arc_mutex_increment(), 1);
}

#[test]
fn test_is_send_sync() {
    is_send_sync::<i32>();
}
