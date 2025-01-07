let plugin_name = variable::get("plugin_name");
let project_name = variable::get("project-name");

let plugin_struct_name = to_upper_camel_case(plugin_name);

let lib_name = "lib" + to_snake_case(project_name) + ".so";

variable::set("plugin_struct_name", plugin_struct_name);
variable::set("lib_name", lib_name);