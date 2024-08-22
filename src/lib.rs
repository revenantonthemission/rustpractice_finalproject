use std::thread;
pub struct ThreadPool {
    threads: Vec<thread::JoinHandle<()>>,
}

impl ThreadPool {
    /// 새로운 스레드 풀을 만든다.
    ///
    /// 구조체의 size는 스레드 풀 안에 있는 스레드의 수다.
    ///
    /// # Panic 조건
    ///
    /// `new` 함수는 size가 0일 때 패닉을 일으킨다.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let mut threads = Vec::with_capacity(size);

        for _ in 0..size {
            // 스레드를 만들어 threads 벡터 안에 저장한다.
        }
        ThreadPool { threads }
    }

    pub fn spawn<F, T>(f: F) -> JoinHandle<T>
    where
        F: FnOnce() -> T,
        F: Send + 'static,
        T: Send + 'static,
    {
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
    }
}
