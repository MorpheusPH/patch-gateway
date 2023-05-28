use k8s_openapi::api::core::v1::{
    LoadBalancerIngress, LoadBalancerStatus, Service, ServiceStatus,
};
use kube::{
    api::{Api, PatchParams, Patch},
    Client
};
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "basic")]
struct Params {
    #[structopt(long)]
    pub name: String,

    #[structopt(long)]
    pub namespace: String,

    #[structopt(long)]
    pub ips: Option<Vec<String>>,

    #[structopt(long)]
    pub hostnames: Option<Vec<String>>,

}

fn create_status_patch(ips: Option<Vec<String>>, hostnames: Option<Vec<String>>) -> Service {
    let mut load_balancer_ingress: Vec<LoadBalancerIngress> = Vec::new();

    if let Some(ips) = ips {
        load_balancer_ingress =
            ips.into_iter()
            .map(|ip| LoadBalancerIngress {
                ip: Some(ip),
                ..Default::default()
            })
            .collect()
    }
    else if let Some(hostnames) = hostnames {
        load_balancer_ingress = 
            hostnames.into_iter()
            .map(|hostname| LoadBalancerIngress {
                hostname: Some(hostname),
                ..Default::default()
            })
            .collect()
    }

    Service {
        status: Some(ServiceStatus {
            load_balancer: Some(LoadBalancerStatus {
                ingress: Some(load_balancer_ingress),
            }),
            ..Default::default()
        }),
        ..Default::default()
    }
}

#[tokio::main]
async fn main() {
    let Params { name, namespace, ips, hostnames } = Params::from_args();

    if ips.is_some() && hostnames.is_some() {
        panic!("`ips` and `hostnames` can only be chosen one at a time");
    }

    if !ips.is_some() && !hostnames.is_some() {
        panic!("`ips` and `hostnames` must choose one");
    }

    let client = Client::try_default().await.unwrap();
    let svcs: Api<Service> = Api::namespaced(client, &namespace);

    let patch = create_status_patch(ips, hostnames);
    let patch_params = PatchParams {
        field_manager: Some("patch-gateway".to_owned()),
        ..Default::default()
    };

    svcs.patch_status(&name, &patch_params, &Patch::Merge(&patch))
        .await
        .unwrap();
}
