fn main() {}

use std::collections::HashMap;

struct AuthenticationManager {
    lifetime: i32,
    records: HashMap<String, i32>,
}

impl AuthenticationManager {
    fn new(timeToLive: i32) -> Self {
        AuthenticationManager {
            lifetime: timeToLive,
            records: HashMap::new(),
        }
    }

    fn generate(&mut self, token_id: String, current_time: i32) {
        self.records.insert(token_id, current_time + self.lifetime);
    }

    fn renew(&mut self, token_id: String, current_time: i32) {
        let eof = match self.records.get_mut(&token_id) {
            None => {
                return;
            }
            Some(t) => t,
        };

        if *eof <= current_time {
            self.records.remove(&token_id);
            return;
        }

        *eof = current_time + self.lifetime;
    }

    fn count_unexpired_tokens(&self, current_time: i32) -> i32 {
        let mut cnt = 0;
        for eof in self.records.values() {
            if eof > &current_time {
                cnt += 1;
            }
        }
        cnt
    }
}

#[test]
fn test_authentication_manager() {
    let mut auth_mgr = AuthenticationManager::new(5);
    auth_mgr.renew("aaa".to_string(), 1);
    auth_mgr.generate("aaa".to_string(), 2);
    assert_eq!(auth_mgr.count_unexpired_tokens(6), 1);
    auth_mgr.generate("bbb".to_string(), 7);
    auth_mgr.renew("aaa".to_string(), 8);
    auth_mgr.renew("bbb".to_string(), 10);
    assert_eq!(auth_mgr.count_unexpired_tokens(15), 0);
}
