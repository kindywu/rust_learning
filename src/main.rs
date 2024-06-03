use futures::executor::block_on;

#[allow(async_fn_in_trait)]
pub trait Executor {
    async fn process(self);
}

pub struct Exec {
    data: String,
}

impl Executor for Exec {
    async fn process(self) {
        println!("hello {}", self.data)
    }
}

fn main() {
    let exec = async {
        let exe = Exec {
            data: "kindy".to_string(),
        };
        exe.process().await
    };

    block_on(exec)
}
