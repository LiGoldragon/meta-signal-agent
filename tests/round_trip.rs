use dotos::{DotosEncode, DotosSource};
use meta_signal_agent::*;

fn exchange() -> signal_frame::ExchangeIdentifier {
    signal_frame::ExchangeIdentifier::new(
        signal_frame::SessionEpoch::new(1),
        signal_frame::ExchangeLane::Connector,
        signal_frame::LaneSequence::first(),
    )
}

fn configuration() -> z2VUoM {
    z2VUoM {
        field_0: z2Vdno::new("deepseek".to_owned()),
        field_1: z2VQu1::new("https://api.deepseek.com".to_owned()),
        field_2: z2VQyu::new("deepseek-chat".to_owned()),
        field_3: z2Veh3::z2Vezi,
    }
}

#[test]
fn authority_projected_request_round_trips_through_dotos_and_the_bound_frame() {
    let request = z2VU7B::z2VWwB(z2VX7d::new(configuration()));
    let text = request.to_dotos();
    assert!(text.starts_with("ConfigureProvider."), "{text}");
    assert_eq!(
        DotosSource::new(&text)
            .parse::<z2VU7B>()
            .expect("request Dotos decodes"),
        request,
    );

    let encoded = request
        .clone()
        .encode_request_frame(exchange())
        .expect("request frame encodes");
    let (decoded_exchange, decoded) =
        ContractMarker::decode_single_request(&encoded).expect("request frame decodes");
    assert_eq!(decoded_exchange, exchange());
    assert_eq!(decoded, request);
}

#[test]
fn authority_projected_reply_round_trips_through_dotos_and_archive_storage() {
    let reply = z2VUiq::z2VUhK(z2VQtu::new(z2Vdno::new("deepseek".to_owned())));
    let text = reply.to_dotos();
    assert!(text.starts_with("ProviderConfigured."), "{text}");
    assert_eq!(
        DotosSource::new(&text)
            .parse::<z2VUiq>()
            .expect("reply Dotos decodes"),
        reply,
    );

    let archive = rkyv::to_bytes::<rkyv::rancor::Error>(&reply).expect("reply archives");
    let recovered =
        rkyv::from_bytes::<z2VUiq, rkyv::rancor::Error>(&archive).expect("reply recovers");
    assert_eq!(recovered, reply);
}
