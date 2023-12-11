(module
    (memory (export "memory") 2 3)  ;; initially 2 pages (2 * 64 kB), maximally 3 pages

    (func (export "write") (param i32)
        (i32.store (local.get 0) (i32.const 1234))
    )
)
