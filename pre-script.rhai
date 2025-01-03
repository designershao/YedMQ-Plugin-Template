let plugin_name = variable::get("plugin_name");

let plugin_struct_name = to_upper_camel_case(plugin_name);

variable::set("plugin_struct_name", plugin_struct_name);

debug(`plugin_name: ${plugin_struct_name}`);