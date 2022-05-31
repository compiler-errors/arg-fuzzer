
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo5457(_: S1, _: S2, _: S4, _: S5, _: S6, _: S7) {}
        
        fn test5457() { foo5457(S7, S6, S1, S5, S6, S0, S6); }
    