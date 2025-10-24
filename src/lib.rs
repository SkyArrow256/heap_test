// 今のところNull文字(\0)で区別してるけど、
// 先頭に読み込むバイト数を置く仕組みの方が
// Null文字を文字列に含められるし、毎回null文字まで走査もしなくていいから
// 効率いいかも！
//
// インタプリタ上でstr + strの計算が起こったとき
// それぞれのポインタの先頭を見るだけで確保するメモリ量も分かるし良さそう。
//
// でもNull文字で文字列管理ってC言語っぽくていいよね。

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
    pub unsafe fn alloc(&mut self, src: &[u8]) -> *mut u8 {
        // Null文字分ひとつ多めに確保
        let layout = Layout::array::<u8>(src.len() + 1).unwrap();
        let p = unsafe { alloc::alloc(layout) };
        if p.is_null() {
            panic!();
        }
        unsafe {
            p.copy_from_nonoverlapping(src.as_ptr(), src.len());
            // null文字を追加
            *p.add(src.len()) = 0;
        }
        self.ptrs.push((p, layout));
        p
    }
    /// ガベージコレクタを起こします。起きろ〜〜〜〜！！！！　ゴミ拾いタイム！！！
    pub unsafe fn gc() {}
    /// 文字列として参照します。
    pub unsafe fn read_as_str<'a>(&'a self, ptr: *const u8) -> &'a str {
        // null文字まで読み込み
        let mut len = 0;
        while let byte = unsafe { ptr.add(len).read() }
            && byte != 0
        {
            len += 1;
        }
        unsafe { str::from_utf8_unchecked(slice::from_raw_parts(ptr, len)) }
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
