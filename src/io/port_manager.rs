use core::arch::asm;
use hashbrown::HashSet;

pub struct PortManager {
    allocated_ports: HashSet<u16>,
}

impl PortManager {
    #[allow(clippy::new_without_default)]
    pub fn new() -> PortManager {
        PortManager {
            allocated_ports: HashSet::new(),
        }
    }

    pub fn request_port(&mut self, addr: u16) -> Option<Port> {
        if self.allocated_ports.contains(&addr) {
            return None;
        }

        self.allocated_ports.insert(addr);

        Some(Port::new(addr))
    }
}

pub struct Port {
    addr: u16,
}

impl Port {
    pub fn new(addr: u16) -> Port {
        Port { addr }
    }

    pub fn get_addr(&self) -> u16 {
        self.addr
    }

    // Store val in reg `al`
    // Store addr in reg `dx`
    // Write the value of `al` to `dx`
    pub fn writeb(&self, val: u8) {
        unsafe {
            asm!(r#"
	        out %al, %dx
	        "#,
                in("dx") self.addr,
                in("al") val,
                options(att_syntax)
            );
        }
    }

    pub fn readb(&self) -> u8 {
        unsafe {
            let mut ret;
            asm!(r#"
                in %dx, %al
                "#,
            in("dx") self.addr,
            out("al") ret,
            options(att_syntax)
            );

            ret
        }
    }
}
