use anyhow::Result;
use yedmq_plugin::{plugin::{AuthenticationResult, AuthenticationResultValue, AuthorizationResult}, register_plugin};

pub struct {{plugin_struct_name}} {}

impl {{plugin_struct_name}} {

    pub fn new(_context: yedmq_plugin::context::Context) -> Result<{{plugin_struct_name}}, anyhow::Error> {
        Ok(
            {{plugin_struct_name}} {}
        )
    }
}

impl yedmq_plugin::plugin::Plugin for {{plugin_struct_name}} {
    fn on_activate(&mut self) -> Result<()> {
        println!("example plugin on_activate {{project-name}}");
        Ok(())
    }

    fn on_deactivate(&self) -> Result<()> {
        println!("example plugin on_deactivate");
        Ok(())
    }

    fn connect_authenticate(&self, _packet: &yedmq_mqtt::v3::connect::ConnectPacket) -> Result<AuthenticationResult> {
        return Ok(AuthenticationResult::Result(AuthenticationResultValue::Success("tenant_id".into())));
    }

    fn authorizate_acl_check(&self, _client: &yedmq_plugin::plugin::Client, _topic: &String, _action: yedmq_plugin::plugin::Action) -> Result<AuthorizationResult> {
        return Ok(AuthorizationResult::Result(true));
    }

    fn on_publish(&self, client: &yedmq_plugin::plugin::Client, packet: &yedmq_mqtt::v3::publish::PublishPacket) {
        println!("client {} receive publish packet {:?}", client.client_identifier, packet)
    }

    fn on_disconnect(&self, client: &yedmq_plugin::plugin::Client) {
        println!("client {} disconnect", client.client_identifier)
    }
}

impl Drop for {{plugin_struct_name}} {
    fn drop(&mut self) {
        println!("example plugin drop");
    }
}

register_plugin!({{plugin_struct_name}}, {{plugin_struct_name}}::new);