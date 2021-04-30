#![allow(missing_docs, unused_variables, trivial_casts)]

extern crate vault_api;
#[allow(unused_extern_crates)]
extern crate futures;
#[allow(unused_extern_crates)]
extern crate swagger;
#[allow(unused_extern_crates)]
extern crate uuid;
extern crate clap;
extern crate url;

#[allow(unused_imports)]
use futures::{Future, future, Stream, stream};
#[allow(unused_imports)]
use vault_api::{ApiNoContext, ContextWrapperExt,
                      ApiError,
                      SysLeasesRevokePutResponse,
                      GenerateCertResponse,
                      ReadCertResponse,
                      CreateOrphanTokenResponse,
                      CreateTokenResponse,
                      LogInWithTLSCertificateResponse,
                      RenewOwnTokenResponse
                     };
use clap::{App, Arg};
use url::Url;

fn main() {
    let matches = App::new("client")
        .arg(Arg::with_name("operation")
            .help("Sets the operation to run")
            .possible_values(&[
    "ReadCert",
    "LogInWithTLSCertificate",
])
            .required(true)
            .index(1))
        .arg(Arg::with_name("https")
            .long("https")
            .help("Whether to use HTTPS or not"))
        .arg(Arg::with_name("host")
            .long("host")
            .takes_value(true)
            .default_value("localhost")
            .help("Hostname to contact"))
        .arg(Arg::with_name("port")
            .long("port")
            .takes_value(true)
            .default_value("8080")
            .help("Port to contact"))
        .get_matches();

    let is_https = matches.is_present("https");
    let base_url = format!("{}://{}:{}",
                           if is_https { "https" } else { "http" },
                           matches.value_of("host").unwrap(),
                           matches.value_of("port").unwrap());
    let base_url = Url::parse(&base_url).expect("Invalid base url");

    let client = if is_https {
        // Using Simple HTTPS
        vault_api::Client::try_new_https(&base_url, "examples/ca.pem")
            .expect("Failed to create HTTPS client")
    } else {
        // Using HTTP
        vault_api::Client::try_new_http(&base_url)
            .expect("Failed to create HTTP client")
    };

    // Using a non-default `Context` is not required; this is just an example!
    let client = client.with_context(vault_api::Context::new_with_span_id(self::uuid::Uuid::new_v4().to_string()));

    match matches.value_of("operation") {

        // Disabled because there's no example.
        // Some("SysLeasesRevokePut") => {
        //     let result = client.sys_leases_revoke_put("x_vault_token_example".to_string(), ???).wait();
        //     println!("{:?} (X-Span-ID: {:?})", result, client.context().x_span_id.clone().unwrap_or(String::from("<none>")));
        //  },

        // Disabled because there's no example.
        // Some("GenerateCert") => {
        //     let result = client.generate_cert("x_vault_token_example".to_string(), "mount_example".to_string(), "name_example".to_string(), ???).wait();
        //     println!("{:?} (X-Span-ID: {:?})", result, client.context().x_span_id.clone().unwrap_or(String::from("<none>")));
        //  },

        Some("ReadCert") => {
            let result = client.read_cert("mount_example".to_string(), "serial_example".to_string()).wait();
            println!("{:?} (X-Span-ID: {:?})", result, client.context().x_span_id.clone().unwrap_or(String::from("<none>")));
         },

        // Disabled because there's no example.
        // Some("CreateOrphanToken") => {
        //     let result = client.create_orphan_token("x_vault_token_example".to_string(), ???).wait();
        //     println!("{:?} (X-Span-ID: {:?})", result, client.context().x_span_id.clone().unwrap_or(String::from("<none>")));
        //  },

        // Disabled because there's no example.
        // Some("CreateToken") => {
        //     let result = client.create_token("x_vault_token_example".to_string(), ???).wait();
        //     println!("{:?} (X-Span-ID: {:?})", result, client.context().x_span_id.clone().unwrap_or(String::from("<none>")));
        //  },

        Some("LogInWithTLSCertificate") => {
            let result = client.log_in_with_tls_certificate(None).wait();
            println!("{:?} (X-Span-ID: {:?})", result, client.context().x_span_id.clone().unwrap_or(String::from("<none>")));
         },

        // Disabled because there's no example.
        // Some("RenewOwnToken") => {
        //     let result = client.renew_own_token("x_vault_token_example".to_string(), ???).wait();
        //     println!("{:?} (X-Span-ID: {:?})", result, client.context().x_span_id.clone().unwrap_or(String::from("<none>")));
        //  },

        _ => {
            panic!("Invalid operation provided")
        }
    }
}

