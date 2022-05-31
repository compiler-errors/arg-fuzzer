
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo2406(_: S1, _: S2, _: S3) {}
        
        fn test2406() { foo2406(S7, S4, S1, S4); }
    