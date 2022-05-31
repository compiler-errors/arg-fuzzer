
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo5626(_: S4, _: S3, _: S3) {}
        
        fn test5626() { foo5626(S1, S2, S4, S5, S6, S7, S8); }
    