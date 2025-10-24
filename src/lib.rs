// 今のところNull文字(\0)で区別してるけど、
// 先頭に読み込むバイト数を置く仕組みの方が
// Null文字を文字列に含められるし、毎回null文字まで走査もしなくていいから
// 効率いいかも！
//
// インタプリタ上でstr + strの計算が起こったとき
// それぞれのポインタの先頭を見るだけで確保するメモリ量も分かるし良さそう。
// しかし、先頭で何バイト使うかによって文字列のサイズに上限が生まれちゃうな...?
//
// Null文字で文字列管理もC言語っぽくていいよね。

use std::{
    alloc::{self, Layout},
    slice,
};

pub struct Heap {
    ptrs: Vec<(*mut u8, Layout)>,
}

impl Heap {
    /// 新しくHeapを作成します。
    pub fn new() -> Self {
        Self { ptrs: Vec::new() }
    }

    /// メモリを確保してバイト列を書き込みます。可変アドレス値が戻ります。
    /// Null文字が最後に入ります。
    pub unsafe fn add(&mut self, src: &[u8]) -> *mut u8 {
        // Null文字分ひとつ多めに確保
        let layout = Layout::array::<u8>(src.len() + 1).unwrap();
        let ptr = unsafe { self.alloc(layout) };
        unsafe {
            ptr.copy_from_nonoverlapping(src.as_ptr(), src.len());
            *ptr.add(src.len()) = 0; // null文字を追加
        }
        ptr
    }

    /// Layoutを受けっとてメモリを確保し、ポインタを返します。
    unsafe fn alloc(&mut self, layout: Layout) -> *mut u8 {
        let ptr = unsafe { alloc::alloc(layout) };
        if ptr.is_null() {
            panic!();
        }
        self.ptrs.push((ptr, layout));
        ptr
    }

    /// 文字列として参照します。
    pub unsafe fn read_as_str<'a>(&'a self, ptr: *const u8) -> &'a str {
        let len = unsafe { Self::strlen(ptr) };
        unsafe { str::from_utf8_unchecked(slice::from_raw_parts(ptr, len)) }
    }

    /// ポインタから文字列長を取得します。
    unsafe fn strlen(ptr: *const u8) -> usize {
        // null文字まで読み込み
        let mut len = 0;
        while let byte = unsafe { ptr.add(len).read() }
            && byte != 0
        {
            len += 1;
        }
        len
    }

    /// 文字列同士を結合して、結合後の文字列へのポインタを取得します。
    pub unsafe fn strcat(&mut self, lhs: *const u8, rhs: *const u8) -> *const u8 {
        let (lhs_len, rhs_len) = unsafe { (Self::strlen(lhs), Self::strlen(rhs)) };
        let layout = Layout::array::<u8>(lhs_len + rhs_len + 1).unwrap();
        let new_ptr = unsafe { self.alloc(layout) };
        unsafe {
            new_ptr.copy_from_nonoverlapping(lhs, lhs_len);
            new_ptr.add(lhs_len).copy_from_nonoverlapping(rhs, rhs_len);
            *new_ptr.add(layout.size()) = 0;
        }
        new_ptr
    }

    /// ガベージコレクタを起こします。起きろ〜〜〜〜！！！！　ゴミ拾いタイム！！！
    pub unsafe fn gc() {
        todo!()
    }
}

impl Drop for Heap {
    /// ドロップ時にメモリを解放します。
    fn drop(&mut self) {
        self.ptrs.iter().for_each(|(ptr, layout)| {
            let slice = unsafe { std::slice::from_raw_parts(*ptr, layout.size()) };
            print!("{:?} at ", slice);
            unsafe { alloc::dealloc(*ptr, *layout) };
            println!("{ptr:?}: free");
        });
    }
}
