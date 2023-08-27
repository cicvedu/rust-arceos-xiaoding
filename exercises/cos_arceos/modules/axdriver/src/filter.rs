#![allow(unused_imports)]

use crate::virtio::VirtIoHalImpl;

use super::prelude::*;
use cfg_if::cfg_if;



pub struct NetFilter<T> {
    pub inner: T,
}

cfg_if! {
    if #[cfg(bus = "pci")] {
        use driver_pci::{PciRoot, DeviceFunction, DeviceFunctionInfo};
        type VirtIoTransport = driver_virtio::PciTransport;
    } else if #[cfg(bus = "mmio")] {
        type VirtIoTransport = driver_virtio::MmioTransport;
    }
}

cfg_if! {
    if #[cfg(net_dev = "virtio-net")] {
        impl BaseDriverOps for NetFilter<driver_virtio::VirtIoNetDev<VirtIoHalImpl, VirtIoTransport, 64>> {
            /// The name of the device.
            fn device_name(&self) -> &str {
                "net-filter"
            }

            /// The type of the device.
            fn device_type(&self) -> driver_common::DeviceType {
                driver_common::DeviceType::Net
            }

        }

        impl NetDriverOps for NetFilter<driver_virtio::VirtIoNetDev<VirtIoHalImpl, VirtIoTransport, 64>> {
            fn mac_address(&self) -> driver_net::EthernetAddress {
                self.inner.mac_address()
            }

            fn can_transmit(&self) -> bool {
                self.inner.can_transmit()
            }

            fn can_receive(&self) -> bool {
                self.inner.can_receive()
            }

            fn rx_queue_size(&self) -> usize {
                self.inner.rx_queue_size()
            }

            fn tx_queue_size(&self) -> usize {
                self.inner.tx_queue_size()
            }

            fn recycle_rx_buffer(&mut self, rx_buf: driver_net::NetBufPtr) -> DevResult {
                 self.inner.recycle_rx_buffer(rx_buf)
            }

            fn recycle_tx_buffers(&mut self) -> DevResult {
                self.inner.recycle_tx_buffers()
            }

            fn transmit(&mut self, tx_buf: driver_net::NetBufPtr) -> DevResult {
                log::warn!("Filter: transmit len[{}]", tx_buf.packet_len());
                self.inner.transmit(tx_buf)
            }

            fn receive(&mut self) -> DevResult<driver_net::NetBufPtr> {
                let receive = self.inner.receive()?;
                log::warn!("Filter: receve len[{:?}]", receive.packet_len());
                Ok(receive)
            }

            fn alloc_tx_buffer(&mut self, size: usize) -> DevResult<driver_net::NetBufPtr> {
                self.inner.alloc_tx_buffer(size)
            } 
        }
    }
}