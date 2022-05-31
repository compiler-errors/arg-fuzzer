
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo2175(_: S1, _: S3, _: S5, _: S6, _: S7) {}
        
        fn test2175() { foo2175(S7, S6, S8, S6, S7, S3, S7); }
    