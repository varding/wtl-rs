


#[cfg(target_arch = "x86")]
pub mod imp {
	use std;
	use winapi::{USHORT,ULONG64};
	//use kernel32;

	#[repr(C,packed)] #[derive(Default,Debug)]
	pub struct Thunk {
	    m_mov:DWORD,          // mov dword ptr [esp+0x4], pThis (esp+0x4 is hWnd)
		m_this:DWORD,         //
		m_jmp:BYTE,          // jmp WndProc
		m_relproc:DWORD,     // relative jmp
	}

	impl Thunk {
		pub fn print(&self){
			println!("print x86:{}",std::mem::size_of::<Thunk>());
		}
	}
}

#[cfg(target_arch = "x86")]
mod tests{
	use super::imp::*;
	#[test]
	fn print(){
		let t = Thunk::default();
		t.print();
		println!("{:?}", t);
	}
}
