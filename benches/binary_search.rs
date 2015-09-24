#![feature(test,asm,rc_counts)]
 
extern crate test;
extern crate rand;
extern crate winapi;

use test::Bencher;
use rand::Rng;
use std::rc::Rc;
use winapi::HWND;
use std::cmp::Ordering;

static test_len:usize = 1000;
static test_linear_key:u32 = 500;
static test_binary_key:u32 = 200;


/*
////////////// Think Pad,R400 Core2 Duo CPU P8600 (2.4G 2.4G) x64 win7 RAM 8G
test_len = 1000;
linear_key = 500;
binary_key = 900;
in combine bench: bin search cnt = log2(n) - 5,that means when length < 32 the linear search is used
running 9 tests
test bench_binary_search              ... bench:          42 ns/iter (+/- 2)
test bench_binary_search_lib          ... bench:          40 ns/iter (+/- 3)
test bench_binary_search_unsafe       ... bench:          41 ns/iter (+/- 2)
test bench_combine_search             ... bench:          33 ns/iter (+/- 1)
test bench_combine_search_foo         ... bench:          33 ns/iter (+/- 1)
test bench_linear_search              ... bench:         637 ns/iter (+/- 38)
test bench_linear_sentinel            ... bench:         431 ns/iter (+/- 56)
test bench_linear_sentinel_unrolling4 ... bench:         427 ns/iter (+/- 29)
test bench_linear_sentinel_unrolling8 ... bench:         421 ns/iter (+/- 94)

test_len = 100;
linear_key = 50;
binary_key = 20;

running 6 tests
test bench_binary_search              ... bench:          27 ns/iter (+/- 3)
test bench_binary_search_unsafe       ... bench:          25 ns/iter (+/- 1)
test bench_linear_search              ... bench:          65 ns/iter (+/- 10)
test bench_linear_sentinel            ... bench:          44 ns/iter (+/- 16)
test bench_linear_sentinel_unrolling4 ... bench:          44 ns/iter (+/- 3)
test bench_linear_sentinel_unrolling8 ... bench:          43 ns/iter (+/- 5)


test_len = 50;
linear_key = 20;
binary_key = 10; //use key from  to 10
running 6 tests
test bench_binary_search              ... bench:          25 ns/iter (+/- 9)
test bench_binary_search_unsafe       ... bench:          21 ns/iter (+/- 0)
test bench_linear_search              ... bench:          27 ns/iter (+/- 2)
test bench_linear_sentinel            ... bench:          18 ns/iter (+/- 0)
test bench_linear_sentinel_unrolling4 ... bench:          19 ns/iter (+/- 1)
test bench_linear_sentinel_unrolling8 ... bench:          19 ns/iter (+/- 1)


test_len = 10;
linear_key = 5;
binary_key = 5;
combine search use binary_key ,so if this value is smaller than linear_key,then combine_search will beats linear_search 
in this situation,that's unfair. Here I chose binary_key = linear_key,but this may affectting the result of binary_search
running 8 tests
test bench_binary_search              ... bench:          14 ns/iter (+/- 1)
test bench_binary_search_lib          ... bench:           3 ns/iter (+/- 0)
test bench_binary_search_unsafe       ... bench:          13 ns/iter (+/- 0)
test bench_combine_search             ... bench:           5 ns/iter (+/- 0)
test bench_linear_search              ... bench:           8 ns/iter (+/- 1)
test bench_linear_sentinel            ... bench:           5 ns/iter (+/- 1)
test bench_linear_sentinel_unrolling4 ... bench:           6 ns/iter (+/- 0)
test bench_linear_sentinel_unrolling8 ... bench:           6 ns/iter (+/- 0)
*/

////////////// i3 cpu/////////////////////////////////////
/*
test_len = 1000;
linear_key = 500;
binary_key = 200;
running 11 tests
test bench_binary_search              ... bench:          21 ns/iter (+/- 0)
test bench_binary_search_cmov         ... bench:          22 ns/iter (+/- 0)
test bench_binary_search_foo          ... bench:          22 ns/iter (+/- 2)
test bench_binary_search_lib          ... bench:          12 ns/iter (+/- 1)
test bench_binary_search_unsafe       ... bench:          20 ns/iter (+/- 1)
test bench_combine_search             ... bench:          12 ns/iter (+/- 1)
test bench_combine_search_foo         ... bench:          15 ns/iter (+/- 0)
test bench_linear_search              ... bench:         293 ns/iter (+/- 32)
test bench_linear_sentinel            ... bench:         228 ns/iter (+/- 10)
test bench_linear_sentinel_unrolling4 ... bench:         173 ns/iter (+/- 3)
test bench_linear_sentinel_unrolling8 ... bench:         168 ns/iter (+/- 2)

test_len = 100;
linear_key = 50;
binary_key = 20;
running 4 tests
test bench_binary_search              ... bench:          13 ns/iter (+/- 1)
test bench_linear_search              ... bench:          35 ns/iter (+/- 1)
test bench_linear_sentinel            ... bench:          21 ns/iter (+/- 0)
test bench_linear_sentinel_unrolling4 ... bench:          19 ns/iter (+/- 0)


test_len = 50;
linear_key = 20;
binary_key = 5; //use key from  to 10
running 4 tests
test bench_binary_search              ... bench:          11 ns/iter (+/- 0)
test bench_linear_search              ... bench:          12 ns/iter (+/- 4)
test bench_linear_sentinel            ... bench:          10 ns/iter (+/- 1)
test bench_linear_sentinel_unrolling4 ... bench:           9 ns/iter (+/- 0)


test_len = 30;
linear_key = 10;
binary_key = 5;
running 4 tests
test bench_binary_search              ... bench:           8 ns/iter (+/- 0)
test bench_linear_search              ... bench:           9 ns/iter (+/- 0)
test bench_linear_sentinel            ... bench:           8 ns/iter (+/- 0)
test bench_linear_sentinel_unrolling4 ... bench:           6 ns/iter (+/- 0)


test_len = 30;
linear_key = 10;
binary_key = 5;
running 5 tests
test bench_binary_search              ... bench:           9 ns/iter (+/- 1)
test bench_linear_search              ... bench:           9 ns/iter (+/- 1)
test bench_linear_sentinel            ... bench:           8 ns/iter (+/- 0)
test bench_linear_sentinel_unrolling4 ... bench:           6 ns/iter (+/- 1)
test bench_linear_sentinel_unrolling8 ... bench:           5 ns/iter (+/- 0)

test_len = 10;
linear_key = 5;
binary_key = 2;
running 11 tests
test bench_binary_search              ... bench:           4 ns/iter (+/- 0)
test bench_binary_search_cmov         ... bench:           5 ns/iter (+/- 1)
test bench_binary_search_foo          ... bench:           8 ns/iter (+/- 0)
test bench_binary_search_lib          ... bench:           3 ns/iter (+/- 0)
test bench_binary_search_unsafe       ... bench:           4 ns/iter (+/- 0)
test bench_combine_search             ... bench:           2 ns/iter (+/- 0)
test bench_combine_search_foo         ... bench:           2 ns/iter (+/- 0)
test bench_linear_search              ... bench:           3 ns/iter (+/- 1)
test bench_linear_sentinel            ... bench:           3 ns/iter (+/- 0)
test bench_linear_sentinel_unrolling4 ... bench:           2 ns/iter (+/- 0)
test bench_linear_sentinel_unrolling8 ... bench:           2 ns/iter (+/- 0)
*/

/*
conclusion:
1. binary search is efficient enough almost all cases ,when array size is small,the search time can be ingored,so we don't need to compare them
2. bound check takes nearly 10% time cost (23-21)/23 = 8.7%
3. unrolling is more important than sentinel in linear search in i3 than P8600
*/

#[bench]
fn bench_linear_search(b: &mut Bencher) {
    let mut v:Vec<u32> = Vec::with_capacity(test_len);
    for i in (0..test_len) {
    	v.push(i as u32);
    }

    b.iter(|| {
    	for i in (0..test_len) {
    		if v[i] >= test_linear_key {
                assert!(v[i] as u32 == test_linear_key);
    			break;
    		}
    	}
    });
}

#[bench]
fn bench_linear_sentinel(b: &mut Bencher) {
    let mut v:Vec<u32> = Vec::with_capacity(test_len+1);
    for i in (0..test_len) {
        v.push(i as u32);
    }

    //push a sentinel as large as possible
    v.push(1<< 31);

    b.iter(|| {
        let mut i = 0;
        loop {
            if v[i] >= test_linear_key {
                break;
            }
            i+=1;
        }
        assert!(v[i] as u32 == test_linear_key);
    });
}

//why unrolling not work?
#[bench]
fn bench_linear_sentinel_unrolling4(b: &mut Bencher) {
    let mut len = test_len;
    let mut ext_len = ((len + 3) /4) * 4; //ceilling
    let mut v:Vec<u32> = Vec::with_capacity(ext_len as usize);
    for i in (0..len) {
        v.push(i as u32);
    }

    //push sentinels as large as possible
    for i in (len..ext_len){
        v.push(1<< 31);
    }

    b.iter(|| {
        let mut i = 0;
        let mut pos = 0;
        loop {
            //unrolling 4 
            if v[i] >= test_linear_key {
                pos = i;
                break;
            }

            if v[i+1] >= test_linear_key {
                pos = i+1;
                break;
            }

            if v[i+2] >= test_linear_key {
                pos = i+2;
                break;
            }

            if v[i+3] >= test_linear_key {
                pos = i+3;
                break;
            }

            i+=4;
        }
        assert!(v[pos] as u32 == test_linear_key);
    });
}

#[bench]
fn bench_linear_sentinel_unrolling8(b: &mut Bencher) {
    let mut len = test_len;
    let mut ext_len = ((len + 3) /8) * 8; //ceilling
    let mut v:Vec<u32> = Vec::with_capacity(ext_len as usize);
    for i in (0..len) {
        v.push(i as u32);
    }

    //push sentinels as large as possible
    for i in (len..ext_len){
        v.push(1<< 31);
    }

    b.iter(|| {
        let mut i = 0;
        let mut pos = 0;
        loop {
            //unrolling 4 
            if v[i] >= test_linear_key {
                pos = i;
                break;
            }

            if v[i+1] >= test_linear_key {
                pos = i+1;
                break;
            }

            if v[i+2] >= test_linear_key {
                pos = i+2;
                break;
            }

            if v[i+3] >= test_linear_key {
                pos = i+3;
                break;
            }

            if v[i+4] >= test_linear_key {
                pos = i+4;
                break;
            }

            if v[i+5] >= test_linear_key {
                pos = i+5;
                break;
            }

            if v[i+6] >= test_linear_key {
                pos = i+6;
                break;
            }

            if v[i+7] >= test_linear_key {
                pos = i+7;
                break;
            }

            i+=8;
        }
        assert!(v[pos] as u32 == test_linear_key);
    });
}

#[bench]
fn bench_binary_search(b: &mut Bencher) {

    let mut v:Vec<u32> = Vec::with_capacity(test_len);
    for i in (0..test_len) {
        v.push(i as u32);
    }

    b.iter(|| {
        let mut left = 0;
        let mut right = test_len-1;
        let mut mid = 0;
        while left < right {
            mid = (left + right) >> 1;
            //debug_assert!(mid < right);
            //key > v[mid],so left = mid + 1
            if v[mid] < test_binary_key {
                left = mid + 1;
            }else{
                //here means v[mid] >= key,v[mid] possibly equal key,so right = mid but not mid - 1
                right = mid;
            }
        }
        assert!((left == right) && (v[left] == test_binary_key));
    });
}

// to avoid bound check
#[bench]
fn bench_binary_search_unsafe(b: &mut Bencher) {

    let mut v:Vec<u32> = Vec::with_capacity(test_len);
    for i in (0..test_len) {
        v.push(i as u32);
    }

    let pt = &v[0] as *const u32;

    b.iter(|| {
        let mut left = 0;
        let mut right = test_len-1;
        let mut mid = 0;
        while left < right {
            mid = (left + right) >> 1;
            assert!(mid < right);
            //key > v[mid],so left = mid + 1
            //if v[mid] < test_binary_key {
            if unsafe{*pt.offset(mid as isize)} < test_binary_key {
                left = mid + 1;
            }else{
                //here means v[mid] >= key,v[mid] possibly equal key,so right = mid but not mid - 1
                right = mid;
            }
        }
        assert!((left == right) && (v[left] == test_binary_key));
    });
}


#[bench]
fn bench_binary_search_lib(b: &mut Bencher) {

    let mut v:Vec<u32> = Vec::with_capacity(test_len);
    for i in (0..test_len) {
        v.push(i as u32);
    }

    b.iter(|| {
        if let Ok(idx) = v.binary_search(&test_binary_key) {
            assert!(v[idx] == test_binary_key);
        }else{
            assert!(false);
        }
    });
}


#[bench]
fn bench_combine_search(b: &mut Bencher) {

    let mut v:Vec<u32> = Vec::with_capacity(test_len);
    for i in (0..test_len) {
        v.push(i as u32);
    }

    //this can be done in the init stage,and use as a const value in the search stage
    let mut bin_search_cnt = (test_len as f32).log2() as u32;

    // if bin_search_cnt < 5,then bin_search_cnt - 5 will be a very big u32 value that equals max_u32 - bin_search_cnt
    if bin_search_cnt > 5 {
        bin_search_cnt -= 5;
    }else{
        bin_search_cnt = 0;
    }

    b.iter(|| {
        let mut left = 0;
        let mut right = test_len-1;
        let mut mid = 0;
        for i in(0..bin_search_cnt){
            mid = (left + right) >> 1;
            //debug_assert!(mid < right);
            //key > v[mid],so left = mid + 1
            if v[mid] < test_binary_key {
                left = mid + 1;
            }else{
                //here means v[mid] >= key,v[mid] possibly equal key,so right = mid but not mid - 1
                right = mid;
            }
        }

        let mut i = left;
        loop{
            if v[i] >= test_binary_key {
                break;
            }
            i+=1;
        }
        assert!(v[i] == test_binary_key);
    });
}

struct Event {
    id: i32,
}

#[repr(C,packed)]
struct Foo {
    msg : u16,
    id  : u16,
    code: u16,
    r   : u16,
    hwnd: HWND,
    //r2  : [u8;64],
    call: Rc<Fn(&Event)->u64>,
}

impl Foo {
    fn new<F>(r:&mut rand::ThreadRng,f:F)->Foo where F:Fn(&Event)->u64 + 'static {
        let h:u64 = r.gen();
        Foo{
            hwnd: h as HWND,
            msg : r.gen(),
            id  : r.gen(),
            code: r.gen(),
            r   : 0,
            //r2  : [10;64],
            call: Rc::new(f),
        }
    }

    #[inline(always)]
    fn data(&self) -> u64 {
        unsafe{
            *(self as *const _ as *const u64)
        }
    }

    fn cmp(&self,other:&Self)->Ordering {
        if self.hwnd < other.hwnd {
            return Ordering::Less;
        }else if self.hwnd > other.hwnd {
            return Ordering::Greater;
        }else{
            return self.data().cmp(&other.data());
        }
    }
}

#[bench]
fn bench_binary_search_foo(b: &mut Bencher) {
    let mut r = rand::thread_rng();
    let mut v:Vec<Foo> = Vec::with_capacity(test_len);
    for i in (0..test_len) {
        v.push(Foo::new(&mut r,|e|100));
    }

    v.sort_by(|f1,f2|{
        //f1.data().cmp(&f2.data())
        f1.cmp(&f2)
    });

    let foo_key = &v[test_binary_key as usize];

    //check rc counter
    for i in (0..test_len){
        assert!(Rc::strong_count(&v[i].call) == 1);    
        assert!(Rc::weak_count(&v[i].call) == 0);    
    }

    b.iter(|| {
        let mut left = 0;
        let mut right = test_len-1;
        let mut mid = 0;
        //for i in(0..bin_search_cnt){
        while left < right{
            mid = (left + right) >> 1;
            //debug_assert!(mid < right);
            //key > v[mid],so left = mid + 1
            //if v[mid].data() < foo_key {
            if v[mid].cmp(&foo_key) == Ordering::Less {
            //if unsafe{(&*pf.offset(mid as isize)).data() < foo_key} {
                left = mid + 1;
            }else{
                //here means v[mid] >= key,v[mid] possibly equal key,so right = mid but not mid - 1
                right = mid;
            }
        }

        let mut i = left;
        // loop{
        //     if v[i].cmp(&foo_key) != Ordering::Less {
        //         break;
        //     }
        //     i+=1;
        // }
        assert!(v[i].cmp(&foo_key) == Ordering::Equal );
    });
}

#[bench]
fn bench_combine_search_foo(b: &mut Bencher) {
    let mut r = rand::thread_rng();
    let mut v:Vec<Foo> = Vec::with_capacity(test_len);
    for i in (0..test_len) {
        v.push(Foo::new(&mut r,|e|100));
    }

    v.sort_by(|f1,f2|{
        //f1.data().cmp(&f2.data())
        f1.cmp(&f2)
    });

    let foo_key = &v[test_binary_key as usize];

    //check rc counter
    for i in (0..test_len){
        assert!(Rc::strong_count(&v[i].call) == 1);    
        assert!(Rc::weak_count(&v[i].call) == 0);    
    }
    
    //this can be done in the init stage,and use as a const value in the search stage
    let mut bin_search_cnt = (test_len as f32).log2() as u32;

    // if bin_search_cnt < 5,then bin_search_cnt - 5 will be a very big u32 value that equals max_u32 - bin_search_cnt
    if bin_search_cnt > 4 {
        bin_search_cnt -= 4;
    }else{
        bin_search_cnt = 0;
    }

    b.iter(|| {
        let mut left = 0;
        let mut right = test_len-1;
        let mut mid = 0;
        for i in(0..bin_search_cnt){
        //while left < right{
            mid = (left + right) >> 1;
            //debug_assert!(mid < right);
            //key > v[mid],so left = mid + 1
            //if v[mid].data() < foo_key {
            if v[mid].cmp(&foo_key) == Ordering::Less {
            //if unsafe{(&*pf.offset(mid as isize)).data() < foo_key} {
                left = mid + 1;
            }else{
                //here means v[mid] >= key,v[mid] possibly equal key,so right = mid but not mid - 1
                right = mid;
            }
        }

        let mut i = left;
        loop{
            if v[i].cmp(&foo_key) != Ordering::Less {
                break;
            }
            i+=1;
        }
        assert!(v[i].cmp(&foo_key) == Ordering::Equal );
    });
}

// the compile already optimized with cmov,so the efficiency of this bench is almost the same with ordinary bin search
#[bench]
fn bench_binary_search_cmov(b: &mut Bencher) {

    let mut v:Vec<u32> = Vec::with_capacity(test_len);
    for i in (0..test_len) {
        v.push(i as u32);
    }

    b.iter(|| {
        let mut left:u32 = 0;
        let mut right:u32 = (test_len-1) as u32;
        let mut mid:u32 = 0;
        while left < right {
            mid = (left + right) >> 1;
            assert!(mid < right);
            unsafe{
                // asm! ("cmpl %3, %2 cmovg %4, %0 cmovle %5, %1"
                //      : "+r" (left), "+r" (right)
                //      : "r" (test_binary_key as u32), "g" (v[mid as usize]), "g" (mid + 1), "g" (mid)
                //      );
                asm!(
                    r"
                    cmp $3, $2;
                    cmovg $4, $0;
                    cmovle $5, $1;
                    "
                    : "+r"(left),"+r" (right)
                    : "r"(test_binary_key), "r"(v[mid as usize]),"r"(mid+1),"r"(mid)
                    : 
                    :
                );
            }
            
        }
        assert!((left == right) && (v[left as usize] == test_binary_key));
    });
}