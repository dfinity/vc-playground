//! Tests related to group management API.
use assert_matches::assert_matches;
use candid::Principal;
use canister_tests::framework::{env, principal_1, principal_2, test_principal};
use ic_canister_sig_creation::IC_ROOT_PK_DER;
use ic_verifiable_credentials::issuer_api::{ArgumentValue, CredentialSpec};
use relying_party::rp_api::{ContentData, ContentError, IssuerData, RpInit, ValidateVpRequest};
use std::collections::{HashMap, HashSet};
use ic_verifiable_credentials::II_ISSUER_URL;

#[allow(dead_code)]
mod util;
use crate::util::{
    api, do_add_exclusive_content, do_list_exclusive_content, do_list_images, install_rp,
};

#[test]
fn should_list_images() {
    let env = env();
    let canister_id = install_rp(&env, None);

    let list = do_list_images(&env, canister_id);
    assert_eq!(list.images.len(), 11);

    let mut img_set = HashSet::new();
    for img_data in &list.images {
        img_set.insert(img_data.url.clone());
    }
    for file_name in [
        "consensus.png",
        "faultTolerance.png",
        "infiniteScalability.png",
        "internetIdentity.png",
        "messageRouting.png",
        "motoko.png",
        "networkNervousSystem.png",
        "peerToPeer.png",
        "protocolUpgrade.png",
        "sdk.png",
        "serviceNervousSystem.png",
    ] {
        assert!(img_set.contains(&format!("/images/{}", file_name)));
    }
    assert_eq!(img_set.len(), 11);
}

#[test]
fn should_add_exclusive_content() {
    let env = env();
    let canister_id = install_rp(&env, None);
    let caller = principal_1();
    let credential_issuer = principal_2();

    let content_name = "Some content name";
    let credential_spec = CredentialSpec {
        credential_type: "VerifiedData".to_string(),
        arguments: None,
    };
    let url = "http://example.com";
    let content_data = do_add_exclusive_content(
        content_name,
        url,
        &credential_spec,
        credential_issuer,
        caller,
        &env,
        canister_id,
    );
    let expected_content_data = ContentData {
        owner: caller,
        content_name: content_name.to_string(),
        created_timestamp_ns: content_data.created_timestamp_ns,
        url: url.to_string(),
        credential_spec,
        credential_issuer,
    };
    assert_eq!(content_data, expected_content_data);
}

#[test]
fn should_list_exclusive_content() {
    let env = env();
    let canister_id = install_rp(&env, None);
    let caller = principal_1();
    let credential_issuer = principal_2();

    let content_name = "Some content name";
    let credential_spec = CredentialSpec {
        credential_type: "VerifiedData".to_string(),
        arguments: None,
    };
    let url = "http://example.com";
    let content_data = do_add_exclusive_content(
        content_name,
        url,
        &credential_spec,
        credential_issuer,
        caller,
        &env,
        canister_id,
    );
    let content_list = do_list_exclusive_content(&env, None, canister_id);
    let expected_content_data = ContentData {
        owner: caller,
        content_name: content_name.to_string(),
        created_timestamp_ns: content_data.created_timestamp_ns,
        url: url.to_string(),
        credential_spec,
        credential_issuer,
    };
    assert_eq!(content_list.content_items.len(), 1);
    assert_eq!(content_list.content_items[0], expected_content_data);
}

#[test]
fn should_list_exclusive_content_multiple_items() {
    let env = env();
    let canister_id = install_rp(&env, None);
    let caller = [principal_1(), principal_2(), test_principal(42)];
    let credential_issuer = principal_2();

    let content_name = [
        "Some content name 1",
        "content name 2",
        "another content name",
    ];
    let credential_spec = [
        CredentialSpec {
            credential_type: "VerifiedData1".to_string(),
            arguments: None,
        },
        CredentialSpec {
            credential_type: "VerifiedData2".to_string(),
            arguments: None,
        },
        CredentialSpec {
            credential_type: "VerifiedData3".to_string(),
            arguments: None,
        },
    ];

    let url = ["http://example_1.com", "other.url", "another url"];
    let mut expected_list = HashMap::new();
    for i in 0..3 {
        let content_data = do_add_exclusive_content(
            content_name[i],
            url[i],
            &credential_spec[i],
            credential_issuer,
            caller[i],
            &env,
            canister_id,
        );
        let expected_content_data = ContentData {
            owner: caller[i],
            content_name: content_name[i].to_string(),
            created_timestamp_ns: content_data.created_timestamp_ns,
            url: url[i].to_string(),
            credential_spec: credential_spec[i].clone(),
            credential_issuer,
        };
        expected_list.insert(content_name[i].to_string(), expected_content_data);
    }
    let content_list = do_list_exclusive_content(&env, None, canister_id);

    assert_eq!(content_list.content_items.len(), content_name.len());
    for item in content_list.content_items {
        let expected = expected_list
            .remove_entry(&item.content_name)
            .expect("Unexpected item");
        assert_eq!(expected.1, item);
    }
}

fn get_validate_vp_request_and_rp_init() -> (ValidateVpRequest, RpInit) {
    let issuer_canister_id =
        Principal::from_text("qdiif-2iaaa-aaaap-ahjaq-cai").expect("wrong principal");
    let ii_canister_id =
        Principal::from_text("fgte5-ciaaa-aaaad-aaatq-cai").expect("wrong principal");
    let issuer_origin = "https://dummy-issuer.vc/";
    let mut args = HashMap::new();
    args.insert("ageAtLeast".to_string(), ArgumentValue::Int(18));

    // Created with Dummy issuer: `https://l7rua-raaaa-aaaap-ahh6a-cai.icp0.io/`
    let req = ValidateVpRequest {
        vp_jwt: "eyJ0eXAiOiJKV1QiLCJhbGciOiJub25lIn0.eyJpc3MiOiJkaWQ6aWNwOnFkaWlmLTJpYWFhLWFhYWFwLWFoamFxLWNhaSIsInZwIjp7IkBjb250ZXh0IjoiaHR0cHM6Ly93d3cudzMub3JnLzIwMTgvY3JlZGVudGlhbHMvdjEiLCJ0eXBlIjoiVmVyaWZpYWJsZVByZXNlbnRhdGlvbiIsInZlcmlmaWFibGVDcmVkZW50aWFsIjpbImV5SnFkMnNpT25zaWEzUjVJam9pYjJOMElpd2lZV3huSWpvaVNXTkRjeUlzSW1zaU9pSk5SSGQzUkVGWlMwdDNXVUpDUVVkRWRVVk5Ra0ZuVFhOQlFXOUJRVUZCUVVGSFFVRktkMFZDWHpKUmNXbGlUMnROYVhKblEwZ3dkMlpZYWtGZmRFbzBiVVF3Y1V0aGVHZDZNM0YyVG5Sc1JtSXlheUo5TENKcmFXUWlPaUprYVdRNmFXTndPbVpuZEdVMUxXTnBZV0ZoTFdGaFlXRmtMV0ZoWVhSeExXTmhhU0lzSW1Gc1p5STZJa2xqUTNNaWZRLmV5SmxlSEFpT2pFM016QTNPVGcxTVRBc0ltbHpjeUk2SW1oMGRIQnpPaTh2YVdSbGJuUnBkSGt1YVdNd0xtRndjQzhpTENKdVltWWlPakUzTXpBM09UYzJNVEFzSW1wMGFTSTZJbVJoZEdFNmRHVjRkQzl3YkdGcGJqdGphR0Z5YzJWMFBWVlVSaTA0TEhScGJXVnpkR0Z0Y0Y5dWN6b3hOek13TnprM05qRXdOVEU0TURnek9EQTJMR0ZzYVdGelgyaGhjMmc2TURGbFpqRmtaRFJrTTJFeE56WTFOV05qWmpKbU5qWmlNamRoTTJVd01UZzNOVEJrT0RGaE5ETm1ZV05rTW1Vek56Qm1NV1UxTW1JellUTXdOekkyTnlJc0luTjFZaUk2SW1ScFpEcHBZM0E2TjJWaWIya3RkSGwxZVhNdFlYRnROR010ZHpKc04ya3RkbWQxWTIwdGVIWmhkM2d0YkdWdGVuZ3RObXR4TW1jdFpqVXpkVGN0ZVhabWFESXRibUZsSWl3aWRtTWlPbnNpUUdOdmJuUmxlSFFpT2lKb2RIUndjem92TDNkM2R5NTNNeTV2Y21jdk1qQXhPQzlqY21Wa1pXNTBhV0ZzY3k5Mk1TSXNJblI1Y0dVaU9sc2lWbVZ5YVdacFlXSnNaVU55WldSbGJuUnBZV3dpTENKSmJuUmxjbTVsZEVsa1pXNTBhWFI1U1dSQmJHbGhjeUpkTENKamNtVmtaVzUwYVdGc1UzVmlhbVZqZENJNmV5SkpiblJsY201bGRFbGtaVzUwYVhSNVNXUkJiR2xoY3lJNmV5SmtaWEpwZG1GMGFXOXVUM0pwWjJsdUlqb2lhSFIwY0hNNkx5OXNOM0oxWVMxeVlXRmhZUzFoWVdGaGNDMWhhR2cyWVMxallXa3VhV013TG1Gd2NDSXNJbWhoYzBsa1FXeHBZWE1pT2lKMmVteGhkaTF4YVdoM1lpMTVkSEJvTWkxdmEyMXZjQzAwTldnMWFpMXBObXBpYUMxcWVHcHBiQzEwTlRSNGJ5MTFjSGN6YkMwMVpHeDRhaTE2WVdVaWZYMTlmUS4yZG4zb210alpYSjBhV1pwWTJGMFpWa0ZiZG5aOTZOa2RISmxaWU1CZ3dHREFZSUVXQ0NtV3ZvNDRpUWlpR09vbnJ0enotVmMtY0MxNXNqNGRXdzVpTTE0T1haZWk0TUNTR05oYm1semRHVnlnd0dEQVlNQmd3R0RBWU1CZ2dSWUlEWWpfU3A3VFNlRGtMU3c0Q1pFbXl3ZEdZM1dGc20tejZ2cGo1a2hKdHlZZ3dHQ0JGZ2dRRUw3S0VUYklHNFhKS0pJN3ZPVDljc2RJaWdQS1kyVWo4R09Da0NGTTBPREFZSUVXQ0NOUGJ4YkdzZ0g2MDh4TzVGeExibFAzMHBRQm9JSGNaOGN1amQzR3lySTc0TUNTZ0FBQUFBQVlBQW5BUUdEQVlNQmd3Sk9ZMlZ5ZEdsbWFXVmtYMlJoZEdHQ0ExZ2cxZGFobzVZVzA0NEx6OTF3S2tQaWdzQWNBbnlCTTVJcW1iRzZ2ajVIa0ppQ0JGZ2diTTFyc3hwVWRoMUtWdW5QMk11amhOVzQtMGNZVG95aFBMY09CUElnbXM2Q0JGZ2dGRm5LMkpEdnF0dS16eFhjVG05ZFMzS21yUlRhMGI4cmwyNG9CcThFMzRhQ0JGZ2dQZWVCM2dnUjlhaEdrV2JGbFBsRFBaWnZhRzlQUUdXdGs1WGpDX3JCVS1LQ0JGZ2d5eXFVQlhBRXJqTnZ0U3Vqa1JmUGtLcXQ3LUF0My1rZ1c4d1R5UFlWQ2dLQ0JGZ2d2Qi1iVEZUMmJyajhKVGdla0dRYTVaNzRmRmtCaGpWUllxVXN0SWRTUXN1Q0JGZ2dETzJzMGF2VnVwMWNaa3hZUTZxVktyTHFYeHI4WFB1ZHB2REc5aUExR1V1Q0JGZ2dkSGxaMDE2Y2VCRHByNWZzSjVDMC1lNElKOUhlSjJCWHJMby1EZC1ZMUw2Q0JGZ2dTS0RIcmMzWTVsMEpDR0R3Zm5TYUFnV0RCWHJDdHY4a0F2aUdlOHUtS1RPREFZSUVXQ0RzVlFoMEZ6MXhHaGZ3MGZzTXBqSmdGRDJQTDhsV2R0UEh1Z1ExSXlISUZJTUNSSFJwYldXQ0EwbktfSU8tbk9qQmdoaHBjMmxuYm1GMGRYSmxXREM1dzM2Vy04RGJVcnBETm14NFBpM3E2UW0wcEdjVkxNTEZiQy1EMlY1R09RR0lNR3NzNlhOWDN3TGpMUV9UbTVacVpHVnNaV2RoZEdsdmJxSnBjM1ZpYm1WMFgybGtXQjBzVmJOSDdQSm9iSU40SFd4WjBiUS1lMHk2amV0c0d6ZGhCX0xOQW10alpYSjBhV1pwWTJGMFpWa0NsTm5aOTZKa2RISmxaWU1CZ2dSWUlDQV9NWmEyWjhmWURoU3dwUENmY19QMFVhMEJ0VWxTZGctamhPLWM5eGZJZ3dHREFZSUVXQ0NkUG5OVjU4dTZpcWxJSWNYZV9rbUQ2bHREY0JId0RlVC1vMDd5cVRacjVJTUNSbk4xWW01bGRJTUJnd0dEQVlNQmdnUllJSWM1LTc3ZFBlMnFqLTlCaHdObndKQmIzamRyWTkwMzRyRjItd2kxZ2dVdmd3R0NCRmdnZ19JMFpfVmVPeWljbnNiS09mNmNBdTlsdlI1dTFjRVlac0txdm5hLXFuYURBWU1DV0Iwc1ZiTkg3UEpvYklONEhXeFowYlEtZTB5NmpldHNHemRoQl9MTkFvTUJnd0pQWTJGdWFYTjBaWEpmY21GdVoyVnpnZ05ZTXRuWjk0S0NTZ0FBQUFBQVlBQUFBUUZLQUFBQUFBQmdBSzRCQVlKS0FBQUFBQUJnQUxBQkFVb0FBQUFBQUdfX193RUJnd0pLY0hWaWJHbGpYMnRsZVlJRFdJVXdnWUl3SFFZTkt3WUJCQUdDM0h3RkF3RUNBUVlNS3dZQkJBR0MzSHdGQXdJQkEyRUFrQWRSSUhlT3NocFRDZ0s4eDJQbjlLR1NrelVHbG1yM3RVd1FwTkt5VGVhb2F5QU9ORUM2NWlaNzlNU0kyYUVkQkhMRGpCdGlJUm1QbU9UbWlDdWppbHBPT3FXdnpvbWJmNEplMlZyZm9TWXBhSUJ6Vlc4blIxSnlFLWpYUGtET2dnUllJRGJ6elNWOWtQczQ1Q1dYOFpPbDRESGIxWVcyS1NlVHV3VGJSNVNBUE9CdWdnUllJSWotb050cDg0LWM4X3VvajRvRUR6eXR5YTUzY3ZvYVFHcHVwR1Q2aFl1ZWdnUllJR2xoN3hOOEt1NExCR2NJTHZiVHdTd0Q2VEFUdGdLa3kySVVKdzVJU0dQeGdnUllJQmNna0pBREU3QTFTWjdLMGQyUU9Vekl6S3VjcnVPcGpBdUZOZnJxdVJwemd3SkVkR2x0WllJRFNmYmtfZXFVdGJTQ0dHbHphV2R1WVhSMWNtVllNSmNRRzRMSUlLRzl5M1BzTkl4cThvalBiTm5fNWNFTkZjWnRNaDdIWHVYNVhwejZZX1JxQWI0Z2R3N3JBYjVvUG1SMGNtVmxnd0dDQkZnZ3dqa1N1bWRHNEE2V0lJenVWdDdhODJIdGxxdlpSNHNRb2VZLTJHN1Y4X21EQWtOemFXZURBbGdnT0psa1V2OWhHVW1jVXVpdlRPczNyRFJKMk1ycFdvR1VUdE1OVVFxYlRnaURBWUlFV0NBbmljaVdSQ1hSNmNJNDluMW1Gbk9aM2k2Y2VTTVZ0czJwZGhLN19PaVpaSU1DV0NDOTdpSDktdTlBT0pCYzhrbUJDaV9vUDROZXJ1c3BzdWVpS1htdUYwNThSNElEUUEiLCJleUpxZDJzaU9uc2lhM1I1SWpvaWIyTjBJaXdpWVd4bklqb2lTV05EY3lJc0ltc2lPaUpOUkhkM1JFRlpTMHQzV1VKQ1FVZEVkVVZOUWtGblRYTkJRVzlCUVVGQlFVRmxRVFpSVVVWQ2JtOXZWVEo1TW1nNU9HUnRMWEZCY25Kc1gxQlZVWFJFUldKcE1VUkNPVlJOVUhOc2NHbGZaMlZzWnlKOUxDSnJhV1FpT2lKa2FXUTZhV053T25Ga2FXbG1MVEpwWVdGaExXRmhZV0Z3TFdGb2FtRnhMV05oYVNJc0ltRnNaeUk2SWtsalEzTWlmUS5leUpsZUhBaU9qRTNNekEzT1RnMU1UUXNJbWx6Y3lJNkltaDBkSEJ6T2k4dlpIVnRiWGt0YVhOemRXVnlMblpqTHlJc0ltNWlaaUk2TVRjek1EYzVOell4TkN3aWFuUnBJam9pWkdGMFlUcDBaWGgwTDNCc1lXbHVPMk5vWVhKelpYUTlWVlJHTFRnc2FYTnpkV1Z5T21oMGRIQnpPaTh2WkhWdGJYa3RhWE56ZFdWeUxuWmpMSFJwYldWemRHRnRjRjl1Y3pveE56TXdOemszTmpFME5qZzJOREF5T0RZNUxITjFZbXBsWTNRNmRucHNZWFl0Y1dsb2QySXRlWFJ3YURJdGIydHRiM0F0TkRWb05Xb3RhVFpxWW1ndGFuaHFhV3d0ZERVMGVHOHRkWEIzTTJ3dE5XUnNlR290ZW1GbElpd2ljM1ZpSWpvaVpHbGtPbWxqY0RwMmVteGhkaTF4YVdoM1lpMTVkSEJvTWkxdmEyMXZjQzAwTldnMWFpMXBObXBpYUMxcWVHcHBiQzEwTlRSNGJ5MTFjSGN6YkMwMVpHeDRhaTE2WVdVaUxDSjJZeUk2ZXlKQVkyOXVkR1Y0ZENJNkltaDBkSEJ6T2k4dmQzZDNMbmN6TG05eVp5OHlNREU0TDJOeVpXUmxiblJwWVd4ekwzWXhJaXdpZEhsd1pTSTZXeUpXWlhKcFptbGhZbXhsUTNKbFpHVnVkR2xoYkNJc0lsWmxjbWxtYVdWa1FXZGxJbDBzSW1OeVpXUmxiblJwWVd4VGRXSnFaV04wSWpwN0lsWmxjbWxtYVdWa1FXZGxJanA3SW1GblpVRjBUR1ZoYzNRaU9qRTRmWDE5ZlEuMmRuM29tdGpaWEowYVdacFkyRjBaVmtHWU5uWjk2TmtkSEpsWllNQmd3R0RBWUlFV0NDbVd2bzQ0aVFpaUdPb25ydHp6LVZjLWNDMTVzajRkV3c1aU0xNE9YWmVpNE1DU0dOaGJtbHpkR1Z5Z3dHREFZSUVXQ0N5WUpjNmV1dVZDS2VnX2hIWU8zY2t6R3poMDhySXg3eXRNemZHelpZQVdvTUJnZ1JZSUJHWTgzUVZYSnNzOGU2bEJxRUlSMVlPRWF6NldQXzZSQUhTb1JyVkVrU1Bnd0dDQkZnZ3lUa21yb2JpbFpWYWh2WERCRWZwUllFS1FsUWdjLUJFdE1LaEFtSlduakNEQVlNQmd3R0NCRmdnU21mcWgzOFJ2ejI0ei0zVTd3TUhiaDU5NmYydFlqQUgwM0pKR3BKSlFzMkRBWUlFV0NDbFpuUXNPNnBzZWRnMjJDYVQxT21KUGtUN1JVOHowUGVvaTRzOFJFWnBSSU1CZ3dHREFZSUVXQ0RKd1BhdlBVWk41UDNaalR2dUI0ck93d2Q1VzhtajloREVFcnJTVWt3RWxZTUJnd0dDQkZnZ05qWVFKdmNfNUdEUGhKd3RpVEFwS0xZUUNaX09TZXpCbHdodWxwOUZua0NEQVlJRVdDRHE4MHZZb0FmOUxhT1ZZMTVFOHJfMndybzVpZERvLUc0WEsxanQyVGJRaklNQmdnUllJSGo2RFdlVFg3N1A3MG5JaWl6LVlsUXoxTHZBRE5WeVBBdHZYT1hFU2RpVGd3SktBQUFBQUFIZ09rRUJBWU1CZ3dHREFrNWpaWEowYVdacFpXUmZaR0YwWVlJRFdDQ2JIUV9aejV5bGJZSEpoN3R2Sl9HaFZubm9DMzdYNHVISHJkOFZtQ2hfOVlJRVdDQ3VTemxvaFVqYTBkci1PNW10M1RWMVRROUZXWExsV0lVWnFzUUVPZ0ZmRVlJRVdDQUpicVVvaUROUzBtbnpMZzY4OGlTejgzelJZTzRCYnFzLTgxbHhPRVhkXzRJRVdDQ3p5U1IwYXdRYkw2OUJQSERvbjRQLW94OXlDckdUZG92N19nR1FTaERCQ0lJRVdDRFgzS0p3YmdYOXNuRDNZT1hHZ0plVERUSFRBakJIMEM4MzByelZ4LUQydG9JRVdDQUtkUnk5RzQ1aGxMaUpKcnFmVHNzejJvcXFIc24xaFdUVUlpVzBLMnN4NDRJRVdDQUxKZFNVNWZPNVE0dlI5cE5wUDVVSEhfV1N1TlBlVDlhWlpuSUV0X2tKR0lJRVdDQWhWWGJ2QXJkZTBDSzFJbEVrM3F3UmNLQm1wUWJOdUFPMUFnQ1VoVEJWeDRJRVdDQkl6SFlqVl9uV2ZId0lNNWE5WFZ4SFpyc0lZbGVucXBuMWkxQU1IQmNacm9JRVdDRGVNUGNrYzFKRWNnZkVFbExEMEFSb1M3WFktbjRnazI5ZDJjMUo3YkI1Q0lNQmdnUllJTEs3VzgxR1Bub0paNGFEc3NWS0lZNVo2LV9WUUlFYVd4dG5IUzVHLTFUWGd3SkVkR2x0WllJRFNiWHk1OHVxNk1HQ0dHbHphV2R1WVhSMWNtVllNS2hBZ0ZldVgxdVF5QkdQX01XRG1RbDRsZVY2QnMydmxNZlJSTnpkbmdTaHNHMlU4bWRIMW9jemRxTTFUN3lIZG1wa1pXeGxaMkYwYVc5dW9tbHpkV0p1WlhSZmFXUllIWk5scU1ueWYwRXdZSVVJTmRHRm1rRmNaMWRjMHd3RW9ZUDlDV01DYTJObGNuUnBabWxqWVhSbFdRSjkyZG4zb21SMGNtVmxnd0dDQkZnZ3ZRNEhOYnN4bkdYTkwwdVhvSUl4ajdFdWdxV21GUGNOSVc5a3BpNkRsdnlEQVlNQmdnUllJTGxKQTJ5d09RbHFuTGYzMzFqcmdFUlJ6VmJBS1FVVkRaTkpGNjB3ZmRoN2d3SkdjM1ZpYm1WMGd3R0RBWUlFV0NDNFM3ZHdZa3JCZEllbjVpSGRjUmJ3LUF6WXZCTWtDOVk2eVBSbVAxX3lVSU1CZ3dHQ0JGZ2daVThpS2cxM2hjbEFhMHpEQ1hEcTVLeE9tU0dScWlDWEFGb3hXVWlzNmR1REFZSUVXQ0RJQ2FMM0hFZzJlZW9JUDhvOXBZTjk0cEJ0NEhNVm5IWnJRNVktbnU3cWFJTUJnZ1JZSUpibzIzeWFLREp1NndlMkJmdTRZSVJZdGc2bUJXcU9GLU1YeGdYMGdPd3Bnd0pZSFpObHFNbnlmMEV3WUlVSU5kR0Zta0ZjWjFkYzB3d0VvWVA5Q1dNQ2d3R0RBazlqWVc1cGMzUmxjbDl5WVc1blpYT0NBMWdiMmRuM2dZSktBQUFBQUFIZ0FBQUJBVW9BQUFBQUFlX19fd0VCZ3dKS2NIVmliR2xqWDJ0bGVZSURXSVV3Z1lJd0hRWU5Ld1lCQkFHQzNId0ZBd0VDQVFZTUt3WUJCQUdDM0h3RkF3SUJBMkVBaXBHeU93bUs2MzNWNVNWeGZ3d3AwNVo4WDA1aDBMWEh4cGNYTG05V2dxcGVRajFDQjNhVnhSZXJwLU8yTnRxVUNUalgxY3J1NG13OGNjeG9pUFZDeFpadVlPcE5rZDI1V3RDWlFvTVhFSmZqSlFKcjd3QXhLTFJHUVFWc2h0QktnZ1JZSU5scEt6TzVOaUhmcGJaQlN1Y1NyUWlDb1BZcGRzTjhVT3JjcFY3MlpvbVFnZ1JZSVBOZURBbnZvcFZsZlpzekFSVF81UlYxdFN0RnNhRFJ1T3c1UmMwTlFnRTFnd0pFZEdsdFpZSURTWkRodnEzbGtPT0JHR2x6YVdkdVlYUjFjbVZZTUpqdHMtMkVKaE5NMTBfc2ZhU3hDQjg0Vm1UWWM1X0FGeWZaYmNwNUVvOUVCOHdHbmlrNW1wVXh2U0NQdnpieXoyUjBjbVZsZ3dKRGMybG5nd0pZSUd1YVFhaWIxMmFnNUxvNWdCOEZLTFlBdGhFNTY3alNWN1B6V0xFdklBc05nd0pZSUQ4WF9xUEtyaUowZ2pXdXFXZUFoZmF6aWhWWFUzcXZVVWhDZE9tNkpZTW9nZ05BIl19fQ.".to_string(),
        effective_vc_subject: Principal::from_text("7eboi-tyuys-aqm4c-w2l7i-vgucm-xvawx-lemzx-6kq2g-f53u7-yvfh2-nae").expect("wrong principal"),
        credential_spec: CredentialSpec { credential_type: "VerifiedAge".to_string(), arguments: Some(args) },
        issuer_origin: issuer_origin.to_string(),
        issuer_canister_id: Some(issuer_canister_id),
    };

    let rp_init = RpInit {
        ic_root_key_der: IC_ROOT_PK_DER.to_vec(),
        ii_vc_url: II_ISSUER_URL.to_string(),
        ii_canister_id,
        issuers: vec![IssuerData {
            vc_url: issuer_origin.to_string(),
            canister_id: issuer_canister_id,
        }],
        derivation_origin: "https://l7rua-raaaa-aaaap-ahh6a-cai.icp0.io".to_string(),
    };
    (req, rp_init)
}

#[test]
fn should_verify_ii_vp() {
    let (req, rp_init) = get_validate_vp_request_and_rp_init();
    // Setup the environment and the RP-canister with the matching configuration.
    let env = env();
    let canister_id = install_rp(&env, Some(rp_init));

    let result =
        api::validate_ii_vp(&env, canister_id, principal_1(), req).expect("API call failed");
    assert!(result.is_ok(), "Validation failed: {:?}", result.err());
}

#[test]
fn should_fail_verify_ii_vp_with_wrong_issuer_origin() {
    let (mut req, rp_init) = get_validate_vp_request_and_rp_init();
    req.issuer_origin = "http://wrong.origin".to_string();
    // Setup the environment and the RP-canister with the matching configuration.
    let env = env();
    let canister_id = install_rp(&env, Some(rp_init));

    let result =
        api::validate_ii_vp(&env, canister_id, principal_1(), req).expect("API call failed");
    assert_matches!(result,
        Err(ContentError::NotAuthorized(e)) if e.contains("issuer not supported"));
}

#[test]
fn should_fail_verify_ii_vp_with_wrong_issuer_canister_id() {
    let (mut req, rp_init) = get_validate_vp_request_and_rp_init();
    req.issuer_canister_id = Some(principal_1());
    // Setup the environment and the RP-canister with the matching configuration.
    let env = env();
    let canister_id = install_rp(&env, Some(rp_init));

    let result =
        api::validate_ii_vp(&env, canister_id, principal_1(), req).expect("API call failed");
    assert_matches!(result,
        Err(ContentError::NotAuthorized(e)) if e.contains("wrong issuer canister id"));
}

#[test]
fn should_fail_verify_ii_vp_with_wrong_effective_vc_subject() {
    let (mut req, rp_init) = get_validate_vp_request_and_rp_init();
    req.effective_vc_subject = principal_1();
    // Setup the environment and the RP-canister with the matching configuration.
    let env = env();
    let canister_id = install_rp(&env, Some(rp_init));

    let result =
        api::validate_ii_vp(&env, canister_id, principal_1(), req).expect("API call failed");
    assert_matches!(result,
        Err(ContentError::NotAuthorized(e)) if e.contains("unexpected vc subject"));
}

#[test]
fn should_fail_verify_ii_vp_with_wrong_credential_spec() {
    let (mut req, rp_init) = get_validate_vp_request_and_rp_init();
    req.credential_spec = CredentialSpec {
        credential_type: "WrongType".to_string(),
        arguments: None,
    };
    // Setup the environment and the RP-canister with the matching configuration.
    let env = env();
    let canister_id = install_rp(&env, Some(rp_init));

    let result =
        api::validate_ii_vp(&env, canister_id, principal_1(), req).expect("API call failed");
    assert_matches!(result,
        Err(ContentError::NotAuthorized(e)) if e.contains("credential_type"));
}

#[test]
fn should_fail_verify_ii_vp_with_invalid_vp() {
    let (mut req, rp_init) = get_validate_vp_request_and_rp_init();
    req.vp_jwt.insert(42, 'a');
    // Setup the environment and the RP-canister with the matching configuration.
    let env = env();
    let canister_id = install_rp(&env, Some(rp_init));

    let result =
        api::validate_ii_vp(&env, canister_id, principal_1(), req).expect("API call failed");
    assert_matches!(result,
        Err(ContentError::NotAuthorized(e)) if e.contains("Failed to parse payload JSON"));
}
