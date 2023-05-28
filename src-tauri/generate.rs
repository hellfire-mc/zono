use zono::rspc::RouterBuilder;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let path = args.as_slice().get(0).map(|s| s.as_str()).unwrap_or("../src/rspc/types.generated.ts");
    RouterBuilder::default().build().export_ts(path).unwrap();
}
