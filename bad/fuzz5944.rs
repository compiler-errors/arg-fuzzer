
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo5944(_: S6, _: S7) {}
        
        fn test5944() { foo5944(S5, S6, S3, S6); }
    