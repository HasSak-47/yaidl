use std::ffi::c_char;

use crate::builder::Code;

#[repr(C)]
pub enum CodeFFI {
    CodeBox(Box<Code>),
    CodeRef(*mut Code),
}

impl CodeFFI {
    fn from_ref(code: *mut Code) -> Self {
        Self::CodeRef(code)
    }

    fn from_code(code: Code) -> Self {
        Self::CodeBox(Box::new(code))
    }

    #[allow(dead_code)]
    #[unsafe(no_mangle)]
    extern "C" fn yaidl_code_new_line(line: *const c_char) -> CodeFFI {
        let line = unsafe { std::ffi::CStr::from_ptr(line).to_str().unwrap().to_string() };
        CodeFFI::from_code(Code::Line(line))
    }

    #[allow(dead_code)]
    #[unsafe(no_mangle)]
    extern "C" fn yaidl_code_new_segment() -> CodeFFI {
        CodeFFI::from_code(Code::new_segment())
    }

    #[allow(dead_code)]
    #[unsafe(no_mangle)]
    extern "C" fn yaidl_code_new_block() -> CodeFFI {
        CodeFFI::from_code(Code::new_block())
    }

    #[allow(dead_code)]
    fn get_code(&mut self) -> &mut Code {
        match self {
            CodeFFI::CodeBox(c) => c,
            CodeFFI::CodeRef(c) => unsafe { &mut **c },
        }
    }

    #[allow(dead_code)]
    fn take_code(self) -> Code {
        match self {
            CodeFFI::CodeBox(c) => *c,
            CodeFFI::CodeRef(_) => unreachable!(),
        }
    }

    #[allow(dead_code)]
    #[unsafe(no_mangle)]
    extern "C" fn yaidl_code_add_child(&mut self, code: CodeFFI) {
        let code = code.take_code();
        match &mut self.get_code() {
            Code::Segment { childs } => childs.push(code),
            Code::Block { childs } => childs.push(code),
            Code::Line(_) => panic!("cannot add childs to a line!"),
        }
    }

    #[allow(dead_code)]
    #[unsafe(no_mangle)]
    extern "C" fn yaidl_code_create_child_segment(&mut self) -> CodeFFI {
        match &mut self.get_code() {
            Code::Segment { childs } => {
                childs.push(Code::new_segment());
                return CodeFFI::from_ref(childs.last_mut().unwrap());
            }
            Code::Block { childs } => {
                childs.push(Code::new_segment());
                return CodeFFI::from_ref(childs.last_mut().unwrap());
            }
            Code::Line(_) => panic!("cannot add line to a line!"),
        }
    }

    #[allow(dead_code)]
    #[unsafe(no_mangle)]
    extern "C" fn yaidl_code_create_child_block(&mut self) -> CodeFFI {
        match self.get_code() {
            Code::Segment { childs } => {
                childs.push(Code::new_block());
                return CodeFFI::from_ref(childs.last_mut().unwrap());
            }
            Code::Block { childs } => {
                childs.push(Code::new_block());
                return CodeFFI::from_ref(childs.last_mut().unwrap());
            }
            Code::Line(_) => panic!("cannot add line to a line!"),
        }
    }

    #[allow(dead_code)]
    #[unsafe(no_mangle)]
    extern "C" fn yaidl_code_add_line(&mut self, line: *const c_char) {
        let line = unsafe { std::ffi::CStr::from_ptr(line).to_str().unwrap().to_string() };

        match self.get_code() {
            Code::Segment { childs } => childs.push(Code::new_line(line)),
            Code::Block { childs } => childs.push(Code::new_line(line)),
            Code::Line(_) => panic!("cannot add line to a line!"),
        }
    }
}
