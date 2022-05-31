
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo2348(_: S1, _: S8) {}
        
        fn test2348() { foo2348(S7, S6, S1, S6); }
    