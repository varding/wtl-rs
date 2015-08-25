

#[cfg(target_arch = "x86_64")]
pub mod imp {
	use std;
	use winapi::{USHORT,ULONG64,c_void,DWORD_PTR,LPCVOID,SIZE_T};
	use kernel32;
	
	#[repr(C)] #[derive(Default,Debug)]
	pub struct Thunk {
		rcx_mov:USHORT,         // mov rcx, pThis
	    rcx_imm:ULONG64,         //
	    rax_mov:USHORT,         // mov rax, target
	    rax_imm:ULONG64,         //
	    rax_jmp:USHORT,         // jmp target
	}

	impl Thunk {
		pub fn print(&self){
			println!("print x64:{}",std::mem::size_of::<Thunk>());
		}

		pub fn init(&mut self,func:DWORD_PTR,p_this:*const c_void){
			self.rcx_mov = 0xb948;          // mov rcx, pThis
        	self.rcx_imm = p_this as ULONG64;  //
        	self.rax_mov = 0xb848;          // mov rax, target
        	self.rax_imm = func as ULONG64;   //
        	self.rax_jmp = 0xe0ff;          // jmp rax
        	unsafe{
        		let p = self as *const Thunk;
        		kernel32::FlushInstructionCache(kernel32::GetCurrentProcess(), p as LPCVOID, std::mem::size_of::<Thunk>() as SIZE_T);
        	}
		}
	}
}


#[cfg(target_arch = "x86_64")]
mod tests{
	#[test]
	fn print(){
		let t = super::imp::Thunk::default();
		t.print();
		println!("{:?}", t);
	}
}
