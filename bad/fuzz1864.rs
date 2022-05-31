
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo1864(_: S3, _: S4, _: S5) {}
        
        fn test1864() { foo1864(S2, S3, S1, S8, S4, S6, S6, S6); }
    