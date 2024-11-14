use bw_r_drivers_tc37x::can::TransmitError;
use bw_r_drivers_tc37x as drivers;
use drivers::pac::can0::{Can0, N as Can0Node};
use drivers::can::pin_map::*;
use drivers::can::config::NodeInterruptConfig;
use drivers::cpu::Priority;
use drivers::can::*;

type CanBase = Node<Can0Node, Can0, Node0, Configured>;

pub enum ErrorTransmit{
    Busy,
    InvalidDataLength,
    InvalidAccess,
}

pub struct CanObj{
    can_node: CanBase,
}

#[allow(unused)]
impl CanObj{
    pub fn init() -> Option<Self>{
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

        Some(Self{can_node: node.lock_configuration()})
    }

    pub fn transmit(&self, frame : &super::frame::Frame) -> Result<(), ErrorTransmit>{
        let frame = (*frame).into();
        match self.can_node.transmit(&frame){
            Ok(_) => Ok(()),
            Err(err) => Err(match err {
                drivers::can::TransmitError::Busy => ErrorTransmit::Busy,
                drivers::can::TransmitError::InvalidDataLength => ErrorTransmit::InvalidDataLength,
                drivers::can::TransmitError::InvalidAccess => ErrorTransmit::InvalidAccess,
            })
        }
    }

    pub fn receive(&self, from: msg::ReadFrom, data: &mut [u8]) -> Option<msg::RxMessage>{
        self.can_node.receive(from, data)
    }
}
