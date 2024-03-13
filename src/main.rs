mod libgraphic;

fn main() {
    pollster::block_on(libgraphic::run());
}
