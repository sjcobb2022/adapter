fn main() {
    println!("Hello, world!");
}

trait Adapter {}
trait Service<M> {}

trait Plugin<A, M>
where
    A: Adapter + Service<M>,
{
}
