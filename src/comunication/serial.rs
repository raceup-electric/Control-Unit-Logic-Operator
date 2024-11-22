use tc375_pac::*;

pub fn uart_init(){
    //asclin register page : 1656 of doc/aurix/Infineon-AURIX_TC3xx_Part2-UserManual-v02_00-EN.pdf
    use tc375_pac::asclin0 as serial_usb; 
    /*
     * Step description to initialize the ASC module with the settings from above: (pag. 1686)
     *
     * 1-   reset ENDINIT to get access to ENDINIT protected register, 
     *      here ASCLIN0_CLC (see also ENDINITprotection chapter.)
     *
     * 2-   enable the control of the module in the clock control register CLC 
     *      (Clock Control Register)
     *
     *  3-  store CLC (Clock Control Register) register in a dummy variable 
     *      (has to be defined before). Read back to avoid pipeline effects.
     *
     *  4-  set ENDINIT to lock the protected register again.
     *
     *  5-  The clk source gets set in the clock selection register CSR (Clock Selection Register). 
     *      Before setting a source, no clk supply (CSR(4:0)=0) must be set. Final clk selection 
     *      in Line 20.
     *
     *  6-  This line sets the loop back mode in the input and output control register IOCR 
     *      (Input and Output Control Register). (see also Figure 416)
     *
     *  7-  This line clears and enables the Tx FIFO and also defines the writing size of 1 
     *      byte per clk. The Tx FIFO filling level to trigger an interrupt gets set to 0 
     *      ([11:8] =0). The Tx FIFO triggers a Tx interrupt, if the Tx FIFO filling 
     *      level falls to or below that defined level. The Tx interrupt get enabled in Line 14. 
     *      (see also section TxFIFO Overview and register TXFIFOCON 
     *      (TX FIFO Configuration Register))
     *
     *  8-  This line installs the Rx FIFO identically like the Tx FIFO from the line above. 
     *      (see also register RXFIFOCON (RX FIFO Configuration Register) 
     *      and section RxFIFO Overview)
     *
     *  9-  The oversampling factor (here 16), the sample points (here 7,8 and 9) and the 
     *      prescaler for the baudrate (here 10) gets configured in the bit configuration register 
     *      BITCON (Bit Configuration Register). (see also section Clock System)
     *
     *  10- One stop bit and initialize mode as basic operation mode (necessary before switching to 
     *      another mode) are getting configured in the FRAMECON (Frame Control Register) register. 
     *      The parity bit feature with parity type even gets also enabled.
     *
     *  11- The data length of 8 bits gets set in the DATCON (Data Configuration Register) register.
     *
     *  12- the clk divider for the baud rate gets set to 48/3125 in the baud rate generation 
     *      register BRG (Baud Rate Generation Register) (see also section Clock System)
     *
     *  13- Clear all interrupt flags in the FLAGSCLEAR (Flags Clear Register) register. 
     *      Before setting the respective interrupt flags in Line 14
     *
     *  14- enable the Tx and Rx interrupts in the FLAGSENABLE (Flags Enable Register) register. 
     *      The interrupts getting triggered by the FIFO’s (see Line 6/7).
     *
     *  15- This line enables the Tx interrupt in the service request control register 
     *      SRC_ASCLIN0TX and sets the interrupt priority to ASC0TX_PRIO (1…255).
     *
     *  16- This line enables the Rx interrupt in the service request control register 
     *      SRC_ASCLIN0RX and sets the interrupt priority to ASC0RX_PRIO (1…255).
     *
     *  17,18-  The interruptHandlerInstall function gets called (this function has to be written 
     *      first, see interrupt handler example for more details). This function installs the 
     *      interrupt service routine (ISR) entry address in the interrupt vector array with 
     *      the priority ASC0TX_PRIO respectively ASC0RX_PRIO.
     *
     *  19- setting operating mode finally to ASC (see also Line 9).
     */

    unsafe {
        //TODO: unlock ENDINIT
        SCU.eicon0().modify(|w| {w.endinit().set(false)});

        ASCLIN0.clc().modify(|w| {w.disr().set(true)});
        ASCLIN0.clc().modify(|w| {w.edis().set(false)});

        SCU.eicon0().modify(|w| {w.endinit().set(true)});
       //TODO: lock ENDINIT

        ASCLIN0.csr().modify(|w| {w.clksel().set(0)});

        ASCLIN0.iocr().modify(|w| {w.lb().set(true)});

        ASCLIN0.txfifocon().modify(|w| {w.flush().set(false)});
        ASCLIN0.txfifocon().modify(|w| {w.inw().set(1)});
        ASCLIN0.txfifocon().modify(|w| {w.intlevel().set(0)});

        ASCLIN0.rxfifocon().modify(|w| {w.flush().set(false)});
        ASCLIN0.rxfifocon().modify(|w| {w.outw().set(1)});
        ASCLIN0.rxfifocon().modify(|w| {w.intlevel().set(0)});

        ASCLIN0.bitcon().modify(|w| {w.oversampling().set(16)});
        ASCLIN0.bitcon().modify(|w| {w.samplepoint().set(9)});
        ASCLIN0.bitcon().modify(|w| {w.prescaler().set(10)});

        ASCLIN0.framecon().modify(|w| {w.stop().set(1)});
        ASCLIN0.framecon().modify(|w| {w.mode().set(0)});
        ASCLIN0.framecon().modify(|w| {w.pen().set(true)});

        ASCLIN0.datcon().modify(|w| {w.datlen().set(8)});

        ASCLIN0.brg().modify(|w| {w.numerator().set(48)});
        ASCLIN0.brg().modify(|w| {w.denominator().set(3125)});

        ASCLIN0.flagsclear().init(|w| {w.set_raw(0)});
        
        ASCLIN0.framecon().modify(|w| {w.mode().set(0b01)});
     }
}
