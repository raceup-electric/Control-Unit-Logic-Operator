/*
 * Author: Alberto Damo
 * Date: 14/11/2024
 *
 * Project Can abstraction. Through the project use ONLY this abstraction to interact with
 * the can. 
 *
 *
 * ---------------------------------------------------------------------------------------
 * DO NOT USE THE SPECIFIC DRIVER/HARDWARE ABSTRACTION. 
 * ---------------------------------------------------------------------------------------
 *
 * If there is a problem with this module, like a missing feature, implement it in here
 * making a wrapper around the driver can abstraction if necessary.
 *
 *
 * By doing that It is possible to change the hardware without rewriting the entire
 * from scratch (look to the old MCU/VCU if you don't believe me!!)
 */

pub mod frame;
pub mod message_id;
pub mod can_obj;
