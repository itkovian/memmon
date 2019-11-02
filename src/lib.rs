/*
Copyright 2019 Andy Georges <itkovian+memmon@gmail.com>

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in
all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
*/

#[macro_use]
extern crate redhook;

//use cgroups::Hierarchy;
use cgroups::cgroup::Cgroup;
use cgroups::cgroup_builder::CgroupBuilder;
use cgroups::memory::MemController;
use libc::c_int;
use std::process;

hook! {

    unsafe fn exit(status: c_int) -> c_int => memmon_exit {
        let hier = cgroups::hierarchies::V1::new();
        let cgroup: Cgroup = CgroupBuilder::new("memmon", &hier)
            .memory()
            .done()
            .build();

        print!("My PID was {}", process::id());

        let mc : &MemController = cgroup.controller_of().expect("No memory controller found");
        let mem = mc.memory_stat();

        println!("Max mem usage: {}", mem.max_usage_in_bytes);

        real!(exit)(status)
    }

}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
