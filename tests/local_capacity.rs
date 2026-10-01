//! Aimemo regression coverage for local files without a ticket-based storage cap.
// SPDX-License-Identifier: Apache-2.0

use memvid_core::{DocMetadata, Memvid, PutOptions, Ticket, Tier};
use tempfile::TempDir;

const MIB: usize = 1024 * 1024;
const FORMER_FREE_CAPACITY: u64 = 50 * MIB as u64;

fn binary_payload(seed: u64, size: usize) -> Vec<u8> {
    let mut payload = vec![0; size];
    fastrand::Rng::with_seed(seed).fill(&mut payload);
    // Keep the binary storage path deterministic and avoid text chunking.
    payload[0] = 0xff;
    payload
}

fn binary_options(uri: String) -> PutOptions {
    PutOptions {
        uri: Some(uri),
        metadata: Some(DocMetadata {
            mime: Some("application/octet-stream".into()),
            ..Default::default()
        }),
        search_text: Some("local capacity regression".into()),
        auto_tag: false,
        extract_dates: false,
        extract_triplets: false,
        instant_index: false,
        ..Default::default()
    }
}

#[test]
fn default_free_file_stores_and_reopens_more_than_fifty_mib() {
    const RECORDS: u64 = 56;
    let dir = TempDir::new().expect("temporary directory");
    let path = dir.path().join("over-fifty-mib.mv2");

    {
        let mut mem = Memvid::create(&path).expect("create default free file");
        assert_eq!(mem.current_ticket().issuer, "free-tier");
        assert_eq!(mem.current_ticket().capacity_bytes, FORMER_FREE_CAPACITY);

        for record in 0..RECORDS {
            let payload = binary_payload(record, MIB);
            mem.put_bytes_with_options(
                &payload,
                binary_options(format!("mv2://capacity/record-{record}.bin")),
            )
            .unwrap_or_else(|err| panic!("insert record {record} beyond the old cap: {err}"));
            // Each entry fits a 2 MiB WAL; committing avoids unrelated WAL growth.
            mem.commit().expect("commit binary record");
        }

        let stats = mem.stats().expect("stats after insertion");
        assert_eq!(stats.tier, Tier::Free);
        assert_eq!(stats.frame_count, RECORDS);
        assert_eq!(stats.payload_bytes, RECORDS * MIB as u64);
        assert!(stats.payload_bytes > FORMER_FREE_CAPACITY);
        assert_eq!(mem.get_capacity(), u64::MAX);
    }

    assert!(std::fs::metadata(&path).expect("file metadata").len() > FORMER_FREE_CAPACITY);
    {
        let mut mem = Memvid::open_read_only(&path).expect("reopen file above old free cap");
        for record in 0..RECORDS {
            let frame = mem
                .frame_by_uri(&format!("mv2://capacity/record-{record}.bin"))
                .expect("find persisted record");
            let actual = mem.frame_canonical_payload(frame.id).expect("read payload");
            assert!(
                actual == binary_payload(record, MIB),
                "payload bytes differ for record {record}"
            );
        }
        assert_eq!(mem.current_ticket().capacity_bytes, FORMER_FREE_CAPACITY);
    }

    // A file that already exceeds the old limit must remain appendable after reopening.
    let tail = binary_payload(RECORDS, 64 * 1024);
    {
        let mut mem = Memvid::open(&path).expect("reopen oversized file for writing");
        mem.put_bytes_with_options(&tail, binary_options("mv2://capacity/tail.bin".into()))
            .expect("append to reopened oversized file");
        mem.commit().expect("commit tail");
    }
    let mut mem = Memvid::open_read_only(&path).expect("reopen after append");
    let stats = mem.stats().expect("final stats");
    assert_eq!(stats.frame_count, RECORDS + 1);
    let frame = mem
        .frame_by_uri("mv2://capacity/tail.bin")
        .expect("find appended record");
    assert!(mem.frame_canonical_payload(frame.id).expect("read tail") == tail);
    println!(
        "free file: {} records, {} payload bytes, {} file bytes; every payload verified after reopening",
        stats.frame_count, stats.payload_bytes, stats.size_bytes
    );
}

#[test]
#[allow(deprecated)] // Construct a legacy persisted ticket using the existing public API.
fn persisted_small_ticket_does_not_restrict_reopened_file() {
    const LEGACY_CAPACITY: u64 = 256 * 1024;
    let dir = TempDir::new().expect("temporary directory");
    let path = dir.path().join("legacy-ticket.mv2");
    let initial = binary_payload(100, 64 * 1024);
    let appended = binary_payload(101, 512 * 1024);

    {
        let mut mem = Memvid::create(&path).expect("create legacy file");
        mem.apply_ticket(Ticket::new("legacy-local-test", 2).capacity_bytes(LEGACY_CAPACITY))
            .expect("persist small legacy ticket");
        mem.put_bytes_with_options(&initial, binary_options("mv2://legacy/initial.bin".into()))
            .expect("write below legacy capacity");
        mem.commit().expect("commit initial payload");
    }

    {
        let mut mem = Memvid::open(&path).expect("reopen legacy file");
        assert_eq!(mem.current_ticket().capacity_bytes, LEGACY_CAPACITY);
        assert_eq!(mem.current_ticket().seq_no, 2);
        mem.put_bytes_with_options(
            &appended,
            binary_options("mv2://legacy/appended.bin".into()),
        )
        .expect("write beyond persisted legacy capacity");
        mem.commit().expect("commit above legacy capacity");
        assert_eq!(mem.get_capacity(), u64::MAX);
    }

    let mut mem = Memvid::open_read_only(&path).expect("reopen above legacy capacity");
    let stats = mem.stats().expect("legacy file stats");
    assert_eq!(stats.frame_count, 2);
    assert!(stats.payload_bytes > LEGACY_CAPACITY);
    assert!(stats.size_bytes > LEGACY_CAPACITY);
    assert_eq!(mem.current_ticket().capacity_bytes, LEGACY_CAPACITY);
    assert_eq!(mem.current_ticket().issuer, "legacy-local-test");
    assert_eq!(mem.current_ticket().seq_no, 2);
    for (uri, expected) in [
        ("mv2://legacy/initial.bin", initial),
        ("mv2://legacy/appended.bin", appended),
    ] {
        let frame = mem.frame_by_uri(uri).expect("find legacy file record");
        let actual = mem.frame_canonical_payload(frame.id).expect("read record");
        assert!(actual == expected, "payload bytes differ for {uri}");
    }
    println!(
        "legacy ticket: {} byte stored cap, {} records, {} payload bytes, {} file bytes; both payloads verified",
        LEGACY_CAPACITY, stats.frame_count, stats.payload_bytes, stats.size_bytes
    );
}
