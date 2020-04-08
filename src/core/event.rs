use model::error::{RdpResult, Error, RdpError, RdpErrorKind};
use num_enum::TryFromPrimitive;
use codec::rle::rle_32_decompress;

/// A bitmap event is used
/// to notify client that it received
/// an old school bitmap data
///
/// If bitmap is compress you can use the
/// decompress function to handle it
pub struct BitmapEvent {
    /// Pixel position from left of the left top angle
    pub dest_left: u16,
    /// Pixel position from top of the left top angle
    pub dest_top: u16,
    /// Pixel position from Right of the left top angle
    pub dest_right: u16,
    /// Pixel position from Bottom of the left top angle
    pub dest_bottom: u16,
    /// width of the bitmap buffer once decompress
    /// This can be larger than dest_right minus dest_left
    pub width: u16,
    /// height of the bitmap buffer once decompress
    /// This can be larger than dest_bottom minus dest_top
    pub height: u16,
    /// Bits Per Pixel
    pub bpp: u16,
    /// true if bitmap buffer is compressed using RLE
    pub is_compress: bool,
    /// Bitmap data
    pub data: Vec<u8>
}

impl BitmapEvent {
    /// Decompress a bitmap which has been encoded by the RLE algorithm
    ///
    /// # Example
    /// ```no_run
    /// use std::net::{SocketAddr, TcpStream};
    /// use rdp::core::client::Connector;
    /// use rdp::core::event::RdpEvent;
    /// let addr = "127.0.0.1:3389".parse::<SocketAddr>().unwrap();
    /// let tcp = TcpStream::connect(&addr).unwrap();
    /// let mut connector = Connector::new()
    ///     .screen(800, 600)
    ///     .credentials("domain".to_string(), "username".to_string(), "password".to_string());
    /// let mut client = connector.connect(tcp).unwrap();
    /// client.read(|rdp_event| {
    ///     match rdp_event {
    ///         RdpEvent::Bitmap(bitmap) => {
    ///             let data = if bitmap.is_compress {
    ///                 bitmap.decompress().unwrap()
    ///             }
    ///             else {
    ///                 bitmap.data
    ///             };
    ///         }
    ///          _ => println!("Unhandled event")
    ///     }
    /// }).unwrap()
    /// ```
    pub fn decompress(&self) -> RdpResult<Vec<u8>> {
        // no compress
        if !self.is_compress {
            return Err(Error::RdpError(RdpError::new(RdpErrorKind::InvalidData, "Trying decompress non compressed image")))
        }

        // actually only handle 32 bpp
        if self.bpp != 32 {
            return Err(Error::RdpError(RdpError::new(RdpErrorKind::NotImplemented, "Decompression Algorithm not implemented")))
        }

        let mut result = vec![0 as u8; self.width as usize * self.height as usize * 4];
        rle_32_decompress(&self.data, self.width as u32, self.height as u32, &mut result)?;
        Ok(result)
    }
}

#[repr(u8)]
#[derive(Eq, PartialEq, TryFromPrimitive, Copy, Clone)]
pub enum PointerButton {
    /// No button but a move
    None = 0,
    /// Left mouse Button
    Left = 1,
    /// Right mouse button
    Right = 2,
    /// Wheel mouse button
    Middle = 3
}

/// A mouse pointer event
pub struct PointerEvent {
    /// horizontal position from top left angle of the window
    pub x: u16,
    /// vertical position from top left angle of the window
    pub y: u16,
    /// Which button is pressed
    pub button: PointerButton,
    /// true if it's a down press action
    pub down: bool
}

/// Keyboard event
/// It's a raw event using Scancode
/// to inform which key is pressed
pub struct KeyboardEvent {
    /// Scancode of the key
    pub code: u16,
    /// State of the key
    pub down: bool
}

/// All event handle by RDP protocol implemented by rdp-rs
pub enum RdpEvent {
    /// Classic bitmap event
    Bitmap(BitmapEvent),
    /// Mouse event
    Pointer(PointerEvent),
    /// Keyboard event
    Key(KeyboardEvent)
}