
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo3373(_: S2, _: S5, _: S7) {}
        
        fn test3373() { foo3373(S3, S3, S4, S6, S0, S5, S6); }
    