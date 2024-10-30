#[cfg(test)]
mod test {
    use fdt_parser::*;

    const TEST_FDT: &[u8] = include_bytes!("../../dtb/bcm2711-rpi-4-b.dtb");

    #[test]
    fn test_find_compatible() {
        let fdt = Fdt::from_bytes(TEST_FDT).unwrap();
        let pl011 = fdt.find_compatible(&["arm,pl011"]).unwrap();
        assert_eq!(pl011.name, "serial@7e201000");
    }

    #[test]
    fn test_find_node1() {
        let fdt = Fdt::from_bytes(TEST_FDT).unwrap();
        let node = fdt.find_node("/soc/timer").unwrap();
        assert_eq!(node.name, "timer@7e003000");
    }
    #[test]
    fn test_find_node2() {
        let fdt = Fdt::from_bytes(TEST_FDT).unwrap();
        let node = fdt.find_node("/soc/serial@7e215040").unwrap();
        assert_eq!(node.name, "serial@7e215040");
    }
    #[test]
    fn test_find_aliases() {
        let fdt = Fdt::from_bytes(TEST_FDT).unwrap();
        let path = fdt.find_aliase("serial0").unwrap();
        assert_eq!(path, "/soc/serial@7e215040");
    }
    #[test]
    fn test_find_node_aliases() {
        let fdt = Fdt::from_bytes(TEST_FDT).unwrap();
        let node = fdt.find_node("serial0").unwrap();
        assert_eq!(node.name, "serial@7e215040");
    }

    #[test]
    fn test_chosen() {
        let fdt = Fdt::from_bytes(TEST_FDT).unwrap();
        let chosen = fdt.chosen().unwrap();
        let bootargs = chosen.bootargs().unwrap();
        assert_eq!(
            bootargs,
            "coherent_pool=1M 8250.nr_uarts=1 snd_bcm2835.enable_headphones=0"
        );

        let stdout = chosen.stdout().unwrap();
        assert_eq!(stdout.params, Some("115200n8"));
        assert_eq!(stdout.node.name, "serial@7e215040");
    }

    #[test]
    fn test_reg() {
        let fdt = Fdt::from_bytes(TEST_FDT).unwrap();
        let node = fdt.find_node("/soc/serial@7e215040").unwrap();

        let reg = node.reg().unwrap().next().unwrap();

        assert_eq!(reg.address, 0xfe215040);
        assert_eq!(reg.child_bus_address, 0x7e215040);
        assert_eq!(reg.size, Some(0x40));
    }

    #[test]
    fn test_interrupt() {
        let fdt = Fdt::from_bytes(TEST_FDT).unwrap();
        let node = fdt.find_node("/soc/serial@7e215040").unwrap();

        let itr_ctrl = node.interrupt_parent().unwrap();

        assert_eq!(itr_ctrl.interrupt_cells(), 3);
    }

    #[test]
    fn test_interrupt2() {
        let fdt = Fdt::from_bytes(TEST_FDT).unwrap();
      

        let node = fdt.find_compatible(&["brcm,bcm2711-hdmi0"]).unwrap();
        let itr_ctrl = node.interrupt_parent().unwrap();

        assert_eq!(itr_ctrl.node.name, "interrupt-controller@7ef00100");
    }
}
