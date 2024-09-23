macro_rules! do_while {
    ( $blk:block while $cond:expr $(;)? ) => {
        while {
            $blk;

            $cond
        } {}
    };
    
    (do $blk:block while $cond:expr $(;)? ) => {
        do_while!($blk while $cond)
    }
}
