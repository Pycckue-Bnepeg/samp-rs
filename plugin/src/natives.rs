use crate::Plugin;
use samp::native;

impl Plugin {
    #[native(name = "GetPlayerIp")]
    pub fn get_player_ip(&self, _amx: usize, _player_id: u16) {
        // samp::amx::current();
    }

    #[native(name = "IsPlayerAdmin")]
    pub fn is_player_admin(&self, _amx: usize, player_id: u16) {
        if let Some(_) = self.admin_list.get(player_id as usize) {}
    }
}
