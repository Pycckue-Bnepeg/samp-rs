use crate::Plugin;
use samp::native;
use samp::amx::Amx;
use samp::error::AmxResult;

impl Plugin {
    #[native(name = "GetPlayerIp")]
    pub fn get_player_ip<'a>(&self, _amx: &'a Amx, _player_id: u16) -> AmxResult<f32> {
        Ok(10.0)
    }

    #[native(name = "IsPlayerAdmin")]
    pub fn is_player_admin(&self, _amx: &Amx, player_id: u16) -> AmxResult<bool> {
        let is_admin = self.admin_list.get(player_id as usize).is_some();
        
        return Ok(is_admin);
    }
}
