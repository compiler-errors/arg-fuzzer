
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo9548(_: S5, _: S3, _: S3, _: S3, _: S7, _: S1) {}
        
        fn test9548() { foo9548(S4, S6, S0, S0, S3, S2, S2); }
    