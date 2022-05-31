
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo5046(_: S6, _: S1, _: S0, _: S6, _: S6) {}
        
        fn test5046() { foo5046(S5, S4, S4, S2, S5, S6, S3); }
    