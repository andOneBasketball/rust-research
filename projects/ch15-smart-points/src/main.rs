use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

#[derive(Debug)]
struct GameState {
    online_players: usize,
}

impl GameState {
    fn new() -> Self {
        GameState { online_players: 0 }
    }

    fn player_join(&mut self) {
        self.online_players += 1;
        println!("A player joined. Total: {}", self.online_players);
    }

    fn player_leave(&mut self) {
        self.online_players -= 1;
        println!("A player left. Total: {}", self.online_players);
    }
}

fn main() {
    // 1. 创建全局共享状态（堆上分配 + 多线程安全）
    let state = Arc::new(Mutex::new(GameState::new()));

    let mut handles = vec![];

    // 2. 模拟 5 个玩家同时加入游戏
    for i in 0..5 {
        let state_clone = Arc::clone(&state);
        let handle = thread::spawn(move || {
            println!("Player {} connecting...", i + 1);

            // 获取互斥锁访问共享状态
            {
                let mut game = state_clone.lock().unwrap();
                game.player_join();
            }

            thread::sleep(Duration::from_millis(500));

            {
                let mut game = state_clone.lock().unwrap();
                game.player_leave();
            }

            println!("Player {} disconnected.", i + 1);
        });

        handles.push(handle);
    }

    // 3. 等待所有线程结束
    for h in handles {
        h.join().unwrap();
    }

    // 4. 主线程读取最终状态
    let final_state = state.lock().unwrap();
    println!("Final Game State: {:?}", *final_state);
}
