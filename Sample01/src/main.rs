use windows::Win32::{
    Foundation::HWND,
    System::{
        DataExchange::{
            CloseClipboard, EnumClipboardFormats, GetClipboardFormatNameA, OpenClipboard,
        },
        Ole::{
            CF_BITMAP, CF_DIB, CF_DIBV5, CF_DIF, CF_DSPBITMAP, CF_DSPENHMETAFILE,
            CF_DSPMETAFILEPICT, CF_DSPTEXT, CF_ENHMETAFILE, CF_GDIOBJFIRST, CF_GDIOBJLAST,
            CF_HDROP, CF_LOCALE, CF_METAFILEPICT, CF_OEMTEXT, CF_OWNERDISPLAY, CF_PALETTE,
            CF_PENDATA, CF_PRIVATEFIRST, CF_PRIVATELAST, CF_RIFF, CF_SYLK, CF_TEXT, CF_TIFF,
            CF_UNICODETEXT, CF_WAVE, CLIPBOARD_FORMAT,
        },
    },
};

fn main() {
    let mut clipboard_format_list = Vec::<CLIPBOARD_FORMAT>::new();

    unsafe {
        match OpenClipboard(HWND::default()) {
            Ok(_s) => {
                let mut available_clipboard_format = CLIPBOARD_FORMAT(0);
                loop {
                    available_clipboard_format = CLIPBOARD_FORMAT(EnumClipboardFormats(
                        available_clipboard_format.0.into(),
                    ) as u16);

                    if available_clipboard_format == CLIPBOARD_FORMAT(0) {
                        break;
                    }

                    clipboard_format_list.push(available_clipboard_format);
                }

                for cbf in &clipboard_format_list {
                    let format_name = get_clipboard_format_name(*cbf)
                        .unwrap_or(format!("0x{:04X}", cbf.0).to_owned());

                    println!("{format_name}");
                }

                let _ = CloseClipboard();
            }
            Err(e) => {
                eprintln!("Failed to open clipboard {:?}", e);
            }
        }
    }
}

fn get_clipboard_format_name(cf: CLIPBOARD_FORMAT) -> Option<String> {
    match cf {
        CF_BITMAP => Some("CF_BITMAP".to_owned()),
        CF_DIB => Some("CF_DIB".to_owned()),
        CF_DIBV5 => Some("CF_DIBV5".to_owned()),
        CF_DIF => Some("CF_DIF".to_owned()),
        CF_DSPBITMAP => Some("CF_DSPBITMAP".to_owned()),
        CF_DSPENHMETAFILE => Some("CF_DSPENHMETAFILE".to_owned()),
        CF_DSPMETAFILEPICT => Some("CF_DSPMETAFILEPICT".to_owned()),
        CF_DSPTEXT => Some("CF_DSPTEXT".to_owned()),
        CF_ENHMETAFILE => Some("CF_ENHMETAFILE".to_owned()),
        CF_GDIOBJFIRST => Some("CF_GDIOBJFIRST".to_owned()),
        CF_GDIOBJLAST => Some("CF_GDIOBJLAST".to_owned()),
        CF_HDROP => Some("CF_HDROP".to_owned()),
        CF_LOCALE => Some("CF_LOCALE".to_owned()),
        CF_METAFILEPICT => Some("CF_METAFILEPICT".to_owned()),
        CF_OEMTEXT => Some("CF_OEMTEXT".to_owned()),
        CF_OWNERDISPLAY => Some("CF_OWNERDISPLAY".to_owned()),
        CF_PALETTE => Some("CF_PALETTE".to_owned()),
        CF_PENDATA => Some("CF_PENDATA".to_owned()),
        CF_PRIVATEFIRST => Some("CF_PRIVATEFIRST".to_owned()),
        CF_PRIVATELAST => Some("CF_PRIVATELAST".to_owned()),
        CF_RIFF => Some("CF_RIFF".to_owned()),
        CF_SYLK => Some("CF_SYLK".to_owned()),
        CF_TEXT => Some("CF_TEXT".to_owned()),
        CF_TIFF => Some("CF_TIFF".to_owned()),
        CF_UNICODETEXT => Some("CF_UNICODETEXT".to_owned()),
        CF_WAVE => Some("CF_WAVE".to_owned()),
        _ => {
            let mut lpsz_registered_format_name = [0u8; 256];
            let registered_format_name_length =
                unsafe { GetClipboardFormatNameA(cf.0 as u32, &mut lpsz_registered_format_name) };

            if registered_format_name_length > 0 {
                let name = String::from_utf8_lossy(
                    &lpsz_registered_format_name[..registered_format_name_length as usize],
                )
                .to_string();
                Some(name)
            } else {
                None
            }
        }
    }
}
