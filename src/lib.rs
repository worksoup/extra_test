use std::collections::HashMap;
use j4rs::{Instance, InvocationArg, Jvm};
use mirai_j4rs::utils::MiraiMap;
use mirai_j4rs::utils::other::enums::MiraiProtocol;

pub struct ExtraTest;

impl ExtraTest {
    pub fn fetch(protocol: &MiraiProtocol, version: String) -> Box<dyn FnOnce(&Jvm, &Instance, &Instance)> {
        Box::new(|jvm: &Jvm, builder: &Instance, config: &Instance| {
            println!("extra test - tmp - fetch");
            let _ = jvm.invoke_static(
                "xyz.cssxsh.mirai.tool.FixProtocolVersion",
                "fetch",
                &[
                    mirai_j4rs::utils::other::protocol_enum_r2j(protocol).unwrap(),
                    InvocationArg::try_from(version).unwrap(),
                ],
            );
        })
    }

    pub fn load(protocol: &MiraiProtocol) -> Box<dyn FnOnce(&Jvm, &Instance, &Instance)> {
        Box::new(|jvm: &Jvm, builder: &Instance, config: &Instance| {
            println!("extra test - tmp - load");
            let _ = jvm.invoke_static(
                "xyz.cssxsh.mirai.tool.FixProtocolVersion",
                "load",
                &[mirai_j4rs::utils::other::protocol_enum_r2j(protocol).unwrap()],
            );
        })
    }

    pub fn load_form_file(protocol: &MiraiProtocol, file: &str) -> Box<dyn FnOnce(&Jvm, &Instance, &Instance)> {
        Box::new(|jvm: &Jvm, builder: &Instance, config: &Instance| {
            println!("extra test - tmp - load");
            let file = InvocationArg::try_from(
                jvm
                    .create_instance("java.io.File", &[InvocationArg::try_from(file).unwrap()])
                    .unwrap(),
            )
                .unwrap();
            let _ = jvm.invoke_static(
                "xyz.cssxsh.mirai.tool.FixProtocolVersion",
                "load",
                &[
                    mirai_j4rs::utils::other::protocol_enum_r2j(protocol).unwrap(),
                    file,
                ],
            );
        })
    }

    pub fn update() -> Box<dyn FnOnce(&Jvm, &Instance, &Instance)> {
        Box::new(|jvm: &Jvm, builder: &Instance, config: &Instance| {
            println!("extra test - tmp - update");
            let _ = jvm
                .invoke_static("xyz.cssxsh.mirai.tool.FixProtocolVersion", "update", &[]);
        })
    }

    pub fn info(jvm: &Jvm) -> HashMap<String, String> {
        println!("extra test - tmp - info");
        let map: MiraiMap<String, String> = MiraiMap::from_instance(
            jvm
                .invoke_static("xyz.cssxsh.mirai.tool.FixProtocolVersion", "info", &[])
                .unwrap(), );
        map.to_hash_map()
    }
}

#[cfg(test)]
mod tests {}
