
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo1406(_: S1, _: S2, _: S3, _: S4, _: S5, _: S7) {}
        
        fn test1406() { foo1406(S5, S3, S1, S2, S6, S4, S8); }
    