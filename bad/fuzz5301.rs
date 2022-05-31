
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo5301(_: S1, _: S6, _: S2, _: S6, _: S4) {}
        
        fn test5301() { foo5301(S4, S5, S7, S3, S2, S8, S1); }
    