use structopt::StructOpt;

pub fn parse_args<ArgsType: StructOpt>(args: &Vec<String>) -> ArgsType {
    // Note: StructOpt wants to eat first argument as program name.
    // But we don't have program name at this point, so we must add fake argument.
    let mut args_copy = args.clone();
    args_copy.insert(0, String::from("--"));

    return ArgsType::from_iter(args_copy.into_iter());
}
