use clap::Parser;
use clvm_tools_rs::classic::clvm_tools::binutils;
use clvm_traits::{clvm_list, clvm_quote, FromClvm, ToClvm};
use clvm_utils::CurriedProgram;
use clvmr::{
    serde::{node_from_bytes, node_to_bytes},
    Allocator, NodePtr,
};

/// Curry a CLVM program in plain text or hex format.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    program: String,
    curried_args: String,

    #[arg(short = 'x', long)]
    hex: bool,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let mut allocator = Allocator::new();

    let program = if args.hex {
        from_hex(&mut allocator, &args.program)?
    } else {
        from_text(&mut allocator, &args.program)?
    };

    let curried_args = if args.hex {
        from_hex(&mut allocator, &args.curried_args)?
    } else {
        from_text(&mut allocator, &args.curried_args)?
    };

    let list = Vec::<NodePtr>::from_clvm(&allocator, curried_args)?;
    let encoded_args = encode_curried_args(&mut allocator, &list)?;

    let curried = CurriedProgram {
        program,
        args: encoded_args,
    }
    .to_clvm(&mut allocator)?;

    println!("{}", to_text(&allocator, curried)?);

    Ok(())
}

fn encode_curried_args(allocator: &mut Allocator, args: &[NodePtr]) -> anyhow::Result<NodePtr> {
    let mut ptr = allocator.one();
    for &arg in args.iter().rev() {
        ptr = clvm_list!(4, clvm_quote!(arg), ptr).to_clvm(allocator)?;
    }
    Ok(ptr)
}

fn from_hex(allocator: &mut Allocator, source: &str) -> anyhow::Result<NodePtr> {
    let source = hex::decode(source.replace("0x", "").replace("0X", ""))?;
    Ok(node_from_bytes(allocator, &source)?)
}

fn from_text(allocator: &mut Allocator, source: &str) -> anyhow::Result<NodePtr> {
    let mut old_allocator = clvmr_old::Allocator::new();
    let input_ptr = binutils::assemble(&mut old_allocator, source)?;
    let input_bytes = clvmr_old::serde::node_to_bytes(&old_allocator, input_ptr)?;
    Ok(node_from_bytes(allocator, &input_bytes)?)
}

fn to_text(allocator: &Allocator, ptr: NodePtr) -> anyhow::Result<String> {
    let mut old_allocator = clvmr_old::Allocator::new();
    let input_bytes = node_to_bytes(allocator, ptr)?;
    let input_ptr = clvmr_old::serde::node_from_bytes(&mut old_allocator, &input_bytes)?;
    Ok(binutils::disassemble(&old_allocator, input_ptr, None))
}
