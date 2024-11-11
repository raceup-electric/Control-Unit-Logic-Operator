use bw_r_drivers_tc37x::embedded_can::{ExtendedId, StandardId};
use bw_r_drivers_tc37x as drivers;
use drivers::can::*;
use drivers::pac::can0::{Can0, N as Can0Node};
use drivers::can::pin_map::*;
use drivers::can::config::NodeInterruptConfig;
use drivers::cpu::Priority;

pub struct CanObj{
    can_node: Option<Node<Can0Node, Can0, Node0, Configured>>,
}

pub struct StandardFrame<'a>{
    base_frame: Frame<'a>
}

impl<'a> drivers::embedded_can::Frame for StandardFrame<'a>{
    fn new(id: impl Into<bw_r_drivers_tc37x::embedded_can::Id>, data: &[u8]) -> Option<Self> {
        let id: drivers::embedded_can::Id = id.into();
        let mx = MessageId::from(id);
        Some(Self{base_frame: Frame::new(mx, data)?})
    }

    fn new_remote(id: impl Into<bw_r_drivers_tc37x::embedded_can::Id>, dlc: usize) -> Option<Self> {
        todo!()
    }

    fn is_extended(&self) -> bool {
        match self.base_frame.id.length{
            msg::MessageIdLength::Standard => false,
            msg::MessageIdLength::Extended | msg::MessageIdLength::Both => true,
        }
    }

    fn is_remote_frame(&self) -> bool {
        todo!()
    }

    fn id(&self) -> bw_r_drivers_tc37x::embedded_can::Id {
        let d = self.base_frame.id.data;
        match self.base_frame.id.length{
            msg::MessageIdLength::Standard => 
                drivers::embedded_can::Id::Standard(StandardId::new(d.try_into().unwrap()).unwrap()),
            msg::MessageIdLength::Extended | msg::MessageIdLength::Both
                => drivers::embedded_can::Id::Extended(ExtendedId::new(d).unwrap()),
        }
    }

    fn dlc(&self) -> usize {
        self.base_frame.data.len()
    }

    fn data(&self) -> &[u8] {
        self.base_frame.data
    }
}

impl CanObj {
    pub fn send_ics_can(frame: &StandardFrame) -> Result<(), ()> {
        // if let Some(n) = self.can_node{
        //     match n.transmit(frame){
        //         Ok(_) => Ok(()),
        //         Err(_) => Err(())
        //     }
        // }else{
        //     Err(())
        // }
        Ok(())
    }

    pub fn new() -> Self {
        Self{
            can_node: None
        }
    }
}


impl super::Ph for CanObj{

    fn init(&mut self) -> Option<()>{
        let can_module = Module::new(Module0);
        let mut can_module = can_module.enable();

        let cfg = NodeConfig {
            baud_rate: BitTimingConfig::Auto(AutoBitTiming {
                baud_rate: 1_000_000,
                sample_point: 8_000,
                sync_jump_width: 3,
            }),
            ..Default::default()
        };

        let mut node = can_module.take_node(Node0, cfg)?;

        node.setup_tx(&TxConfig {
            mode: TxMode::DedicatedBuffers,
            dedicated_tx_buffers_number: 2,
            fifo_queue_size: 0,
            buffer_data_field_size: DataFieldSize::_8,
            event_fifo_size: 1,
            tx_event_fifo_start_address: 0x400,
            tx_buffers_start_address: 0x440,
        });

        node.setup_rx(RxConfig {
            mode: RxMode::SharedFifo0,
            buffer_data_field_size: DataFieldSize::_8,
            fifo0_data_field_size: DataFieldSize::_8,
            fifo1_data_field_size: DataFieldSize::_8,
            fifo0_operating_mode: RxFifoMode::Blocking,
            fifo1_operating_mode: RxFifoMode::Blocking,
            fifo0_watermark_level: 0,
            fifo1_watermark_level: 0,
            fifo0_size: 4,
            fifo1_size: 0,
            rx_fifo0_start_address: 0x100,
            rx_fifo1_start_address: 0x200,
            rx_buffers_start_address: 0x300,
        });

        node.setup_pins(Some(&Pins {
            tx: PIN_TX_0_0_P20_8,
            rx: PIN_RX_0_0_P20_7,
        }));

        node.setup_interrupt(&NodeInterruptConfig {
            interrupt_group: InterruptGroup::Rxf0n,
            interrupt: Interrupt::RxFifo0newMessage,
            line: InterruptLine::Line1,
            priority: Priority::try_from(2).unwrap(),
            tos: Tos::Cpu0,
        });

        self.can_node = Some(node.lock_configuration());
        Some(())
    }
}
