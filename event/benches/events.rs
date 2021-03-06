#[macro_use]
extern crate criterion;

use criterion::Criterion;
use reclutch_event::*;
use std::mem::drop;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("rcevent-listener-peek", move |b| {
        b.iter(|| {
            let event = RcEventQueue::new();

            event.emit_owned(0);

            let listener = event.listen();

            event.emit_owned(1);
            event.emit_owned(2);
            event.emit_owned(3);

            assert_eq!(listener.peek(), &[1, 2, 3]);
        })
    });

    c.bench_function("rcevent-listener-with", move |b| {
        b.iter(|| {
            let event = RcEventQueue::new();

            event.emit_owned(0);

            let listener = event.listen();

            event.emit_owned(1);
            event.emit_owned(2);
            event.emit_owned(3);

            listener.with(|events| {
                assert_eq!(events, &[1, 2, 3]);
            });
        })
    });

    c.bench_function("rcevent-cleanup", move |b| {
        b.iter(|| {
            let event = RcEventQueue::new();

            let listener_1 = event.listen();

            event.emit_owned(10);

            let listener_2 = event.listen();

            event.emit_owned(20);

            assert_eq!(listener_1.peek(), &[10, 20]);
            assert_eq!(listener_2.peek(), &[20]);
            let empty_peeked: &[i32] = &[];
            assert_eq!(listener_2.peek(), empty_peeked);
            assert_eq!(listener_2.peek(), empty_peeked);

            for i in [30; 10].iter() {
                event.emit_borrowed(i);
            }

            assert_eq!(listener_2.peek(), &[30; 10]);

            drop(listener_1);
        })
    });

    c.bench_function("nonrcevent-listener-peek", move |b| {
        b.iter(|| {
            let event = NonRcEventQueue::new();

            event.emit_owned(0);

            let listener = NonRcEventListener::new(&event);

            event.emit_owned(1);
            event.emit_owned(2);
            event.emit_owned(3);

            assert_eq!(listener.peek(), &[1, 2, 3]);
        })
    });

    c.bench_function("nonrcevent-listener-with", move |b| {
        b.iter(|| {
            let event = NonRcEventQueue::new();

            event.emit_owned(0);

            let listener = NonRcEventListener::new(&event);

            event.emit_owned(1);
            event.emit_owned(2);
            event.emit_owned(3);

            listener.with(|events| {
                assert_eq!(events, &[1i32, 2i32, 3i32]);
            });
        })
    });

    c.bench_function("nonrcevent-cleanup", move |b| {
        b.iter(|| {
            let event = NonRcEventQueue::new();

            let listener_1 = NonRcEventListener::new(&event);

            event.emit_owned(10);

            let listener_2 = NonRcEventListener::new(&event);

            event.emit_owned(20);

            assert_eq!(listener_1.peek(), &[10i32, 20i32]);
            assert_eq!(listener_2.peek(), &[20i32]);
            let empty_peeked: &[i32] = &[];
            assert_eq!(listener_2.peek(), empty_peeked);
            assert_eq!(listener_2.peek(), empty_peeked);

            for i in [30; 10].iter() {
                event.emit_borrowed(i);
            }

            assert_eq!(listener_2.peek(), &[30i32; 10]);

            drop(listener_1);
        })
    });

    c.bench_function("rawevent-pull-with", move |b| {
        b.iter(|| {
            let mut event = RawEventQueue::new();

            let listener_1 = event.create_listener();

            event.emit_owned(10);

            let listener_2 = event.create_listener();

            event.emit_owned(20);

            event.pull_with(listener_1, |x| assert_eq!(x, &[10i32, 20i32]));
            event.pull_with(listener_2, |x| assert_eq!(x, &[20i32]));
            let empty_peeked: &[i32] = &[];
            event.pull_with(listener_2, |x| assert_eq!(x, empty_peeked));
            event.pull_with(listener_2, |x| assert_eq!(x, empty_peeked));

            for _i in 0..10 {
                event.emit_owned(30);
            }

            event.pull_with(listener_2, |x| assert_eq!(x, &[30i32; 10]));

            event.remove_listener(listener_1);
        })
    });

    c.bench_function("event-merge-with", move |b| {
        b.iter(|| {
            use reclutch_event::merge::Merge;
            let event1 = RcEventQueue::new();
            let event2 = RcEventQueue::new();
            let eventls: Vec<_> = vec![event1.listen(), event2.listen()]
                .into_iter()
                .map(|i| Box::new(i) as Box<dyn Merge<i32>>)
                .collect();

            event1.emit_owned(0);
            event2.emit_owned(1);
            event1.emit_owned(2);
            event2.emit_owned(3);

            eventls.with(|events| {
                assert_eq!(events, &[0i32, 2, 1, 3]);
            });
        })
    });

    c.bench_function("event-merge-map", move |b| {
        b.iter(|| {
            use reclutch_event::merge::Merge;
            let event1 = RcEventQueue::new();
            let event2 = RcEventQueue::new();
            let eventls: Vec<_> = vec![event1.listen(), event2.listen()]
                .into_iter()
                .map(|i| Box::new(i) as Box<dyn Merge<i32>>)
                .collect();

            event1.emit_owned(0);
            event2.emit_owned(1);
            event1.emit_owned(2);
            event2.emit_owned(3);

            assert_eq!(eventls.map(|&x| x), &[0i32, 2, 1, 3]);
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
