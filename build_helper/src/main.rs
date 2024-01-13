/*
 * Copyright (c) 2024. The RigelA open source project team and
 * its contributors reserve all rights.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 * http://www.apache.org/licenses/LICENSE-2.0
 * Unless required by applicable law or agreed to in writing, software distributed under the
 * License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and limitations under the License.
 */

#[cfg(target_arch = "x86_64")]
fn main() {
    use std::env;
    use std::process::Command;
    let mut arg_list = env::args()
        .skip(2)
        .collect::<Vec<_>>();
    if arg_list.contains(&String::from("--target")) {
        panic!("The --target argument is not allowed.");
    }
    arg_list.push("--target".to_string());

    // 获取cargo的路径
    let cargo = env::var("CARGO")
        .expect("Can't directly run the current program, this program can only be called through cargo.");

    // 先构建32位目标，因为64位主程序需要依赖他
    let args = {
        let mut v = arg_list.clone();
        if v[0] == "run" {
            // 如果是运行命令，就改成构建，因为不需要运行32位的主程序
            v[0] = "build".to_string();
        }
        v.push("i686-pc-windows-msvc".to_string());
        v
    };
    Command::new(cargo.as_str())
        .args(args)
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
    // 然后构建64位目标
    let args = {
        let mut v = arg_list.clone();
        v.push("x86_64-pc-windows-msvc".to_string());
        v
    };
    Command::new(cargo.as_str())
        .args(args)
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
}


#[cfg(target_arch = "x86")]
fn main() {
    panic!("X86 arch target is unsupported!");
}
